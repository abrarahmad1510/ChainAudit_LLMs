use anyhow::{anyhow, Result};
use tonic::transport::Channel;
use crate::config::TrillianConfig;

include!(concat!(env!("OUT_DIR"), "/trillian.rs"));

pub struct TrillianClient {
    client: trillian_log_client::TrillianLogClient<Channel>,
    log_id: i64,
}

#[derive(Debug, Clone)]
pub struct SignedRoot {
    pub root_hash: Vec<u8>,
    pub tree_size: i64,
}

impl TrillianClient {
    pub async fn new(cfg: &TrillianConfig) -> Result<Self> {
        let channel = Channel::from_shared(cfg.log_server_addr.clone())?
            .connect()
            .await?;
        let client = trillian_log_client::TrillianLogClient::new(channel);
        Ok(Self {
            client,
            log_id: cfg.log_id,
        })
    }

    pub async fn queue_leaf(&self, leaf_hash: &[u8]) -> Result<i64> {
        let leaf = LogLeaf {
            leaf_value: leaf_hash.to_vec(),
            extra_data: vec![],
            leaf_identity_hash: vec![],
            ..Default::default()
        };
        let request = QueueLeafRequest {
            log_id: self.log_id,
            leaf: Some(leaf),
            charge_to: None,
        };
        let response = self.client
            .clone()
            .queue_leaf(request)
            .await?
            .into_inner();
        match response.queued_leaf {
            Some(ql) => {
                if let Some(leaf) = ql.leaf.as_ref() {
                    Ok(leaf.leaf_index)
                } else {
                    Err(anyhow!("Queued leaf missing leaf data"))
                }
            }
            None => Err(anyhow!("Leaf not queued")),
        }
    }

    pub async fn get_current_root(&self) -> Result<SignedRoot> {
        let request = GetLatestSignedLogRootRequest {
            log_id: self.log_id,
            charge_to: None,
            first_tree_size: 0,
        };
        let response = self.client
            .clone()
            .get_latest_signed_log_root(request)
            .await?
            .into_inner();
        match response.signed_log_root {
            Some(slr) => {
                // Parse the TLS-serialized log_root
                let log_root_bytes = &slr.log_root;
                if log_root_bytes.len() < 10 {
                    return Err(anyhow!("Invalid log root: too short"));
                }
                
                // Parse TLS format: version(2) + tree_size(8) + hash_len(1) + hash + ...
                let tree_size = i64::from_be_bytes([
                    log_root_bytes[2], log_root_bytes[3], log_root_bytes[4], log_root_bytes[5],
                    log_root_bytes[6], log_root_bytes[7], log_root_bytes[8], log_root_bytes[9],
                ]);
                
                let hash_len = log_root_bytes[10] as usize;
                if log_root_bytes.len() < 11 + hash_len {
                    return Err(anyhow!("Invalid log root: hash too short"));
                }
                
                let root_hash = log_root_bytes[11..11 + hash_len].to_vec();
                
                Ok(SignedRoot {
                    root_hash,
                    tree_size,
                })
            }
            None => Err(anyhow!("No root available")),
        }
    }

    pub async fn get_inclusion_proof(&self, leaf_index: i64, tree_size: i64) -> Result<Vec<Vec<u8>>> {
        let request = GetInclusionProofRequest {
            log_id: self.log_id,
            leaf_index,
            tree_size,
            charge_to: None,
        };
        let response = self.client
            .clone()
            .get_inclusion_proof(request)
            .await?
            .into_inner();
        match response.proof {
            Some(proof) => Ok(proof.hashes),
            None => Err(anyhow!("No inclusion proof available")),
        }
    }
}
