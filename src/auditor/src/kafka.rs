use anyhow::Result;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use crate::config::KafkaConfig;

pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub async fn new(cfg: &KafkaConfig) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &cfg.brokers)
            .set("message.timeout.ms", "5000")
            .create()?;
        Ok(Self {
            producer,
            topic: cfg.topic.clone(),
        })
    }

    pub async fn publish(&self, leaf_hash: &[u8], receipt: &str) -> Result<()> {
        let payload = format!("{}:{}", hex::encode(leaf_hash), receipt);
        let key = hex::encode(leaf_hash); // Store key in a variable to extend its lifetime
        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key(&key);
        self.producer.send(record, std::time::Duration::from_secs(5)).await
            .map_err(|(e, _)| anyhow::anyhow!("Kafka error: {}", e))?;
        Ok(())
    }
}
