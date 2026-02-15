use tonic::{Request, Response, Status, Streaming};
use crate::auditor::{
    auditor_server::{Auditor, AuditorServer},
    HashSubmission, ReceiptResponse, ReceiptRequest,
};
use crate::config::Config;
use crate::storage::Storage;
use crate::trillian::TrillianClient;
use crate::signer::Signer;
use crate::kafka::KafkaProducer;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};
use tokio::time::{self, Duration};
use tracing::{info, error};

pub struct AuditorService {
    storage: Arc<Storage>,
    trillian: Arc<TrillianClient>,
    signer: Arc<Signer>,
    kafka: Arc<KafkaProducer>,
    batch_tx: UnboundedSender<HashSubmission>,
}

#[tonic::async_trait]
impl Auditor for AuditorService {
    type SubmitHashStream = mpsc::Receiver<Result<ReceiptResponse, Status>>;

    async fn submit_hash(
        &self,
        request: Request<Streaming<HashSubmission>>,
    ) -> Result<Response<Self::SubmitHashStream>, Status> {
        let mut stream = request.into_inner();
        let (tx, rx) = mpsc::channel(128);

        let batch_tx = self.batch_tx.clone();
        let this = self.clone(); // clone Arcs for the spawned task

        tokio::spawn(async move {
            while let Some(submission) = stream.next().await {
                match submission {
                    Ok(sub) => {
                        // Send to batching channel
                        if batch_tx.send(sub).is_err() {
                            error!("Batching channel closed");
                            break;
                        }
                    }
                    Err(e) => {
                        error!("Error in submission stream: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(Response::new(mpsc::ReceiverStream::new(rx)))
    }

    async fn get_receipt(
        &self,
        request: Request<ReceiptRequest>,
    ) -> Result<Response<ReceiptResponse>, Status> {
        let leaf_hash = request.into_inner().leaf_hash;
        let receipt = self.storage.get_receipt(&leaf_hash).await
            .map_err(|e| Status::internal(format!("Storage error: {}", e)))?;
        Ok(Response::new(ReceiptResponse {
            receipt: receipt.receipt_jwt.into_bytes(),
            leaf_index: receipt.leaf_index as u64,
        }))
    }
}

impl AuditorService {
    async fn process_single_submission(
        sub: HashSubmission,
        trillian: &TrillianClient,
        signer: &Signer,
        storage: &Storage,
        kafka: &KafkaProducer,
    ) -> anyhow::Result<ReceiptResponse> {
        let leaf_index = trillian.queue_leaf(&sub.hash).await?;
        let signed_root = trillian.get_current_root().await?;
        let inclusion_proof = trillian.get_inclusion_proof(leaf_index, signed_root.tree_size).await?;
        let receipt_jwt = signer.sign_receipt(
            &sub.hash,
            leaf_index,
            &signed_root.root_hash,
            &inclusion_proof,
            &sub.metadata,
        ).await?;
        storage.store_receipt(
            &sub.hash,
            leaf_index,
            &signed_root.root_hash,
            &sub.metadata,
            &receipt_jwt,
        ).await?;
        kafka.publish(&sub.hash, &receipt_jwt).await?;
        Ok(ReceiptResponse {
            receipt: receipt_jwt.into_bytes(),
            leaf_index: leaf_index as u64,
        })
    }
}

impl Clone for AuditorService {
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
            trillian: self.trillian.clone(),
            signer: self.signer.clone(),
            kafka: self.kafka.clone(),
            batch_tx: self.batch_tx.clone(),
        }
    }
}

async fn process_batch(
    batch: Vec<HashSubmission>,
    trillian: Arc<TrillianClient>,
    signer: Arc<Signer>,
    storage: Arc<Storage>,
    kafka: Arc<KafkaProducer>,
) {
    for sub in batch {
        if let Err(e) = AuditorService::process_single_submission(sub, &trillian, &signer, &storage, &kafka).await {
            error!("Failed to process submission in batch: {}", e);
        }
    }
}

pub async fn run(cfg: Config) -> anyhow::Result<()> {
    let storage = Arc::new(Storage::new(&cfg.database.url).await?);
    let trillian = Arc::new(TrillianClient::new(&cfg.trillian).await?);
    let signer = Arc::new(Signer::new(&cfg.sigstore).await?);
    let kafka = Arc::new(KafkaProducer::new(&cfg.kafka).await?);

    // Batching channel
    let (batch_tx, mut batch_rx): (UnboundedSender<HashSubmission>, UnboundedReceiver<HashSubmission>) = mpsc::unbounded_channel();

    let trillian_clone = trillian.clone();
    let signer_clone = signer.clone();
    let storage_clone = storage.clone();
    let kafka_clone = kafka.clone();

    tokio::spawn(async move {
        let mut batch = Vec::new();
        let mut interval = time::interval(Duration::from_millis(100));
        loop {
            tokio::select! {
                Some(sub) = batch_rx.recv() => {
                    batch.push(sub);
                    if batch.len() >= 100 {
                        process_batch(batch, trillian_clone.clone(), signer_clone.clone(), storage_clone.clone(), kafka_clone.clone()).await;
                        batch = Vec::new();
                    }
                }
                _ = interval.tick() => {
                    if !batch.is_empty() {
                        process_batch(batch, trillian_clone.clone(), signer_clone.clone(), storage_clone.clone(), kafka_clone.clone()).await;
                        batch = Vec::new();
                    }
                }
            }
        }
    });

    let service = AuditorService {
        storage,
        trillian,
        signer,
        kafka,
        batch_tx,
    };

    let addr = cfg.server.addr.parse()?;
    info!("Starting auditor server on {}", addr);
    tonic::transport::Server::builder()
        .add_service(AuditorServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
