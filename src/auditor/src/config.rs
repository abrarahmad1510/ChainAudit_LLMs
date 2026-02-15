use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub kafka: KafkaConfig,
    pub trillian: TrillianConfig,
    pub sigstore: SigstoreConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub addr: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KafkaConfig {
    pub brokers: String,
    pub topic: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TrillianConfig {
    pub log_server_addr: String,
    pub log_id: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SigstoreConfig {
    pub fulcio_url: String,
    pub rekor_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let config_path = std::env::var("CONFIG_PATH")
            .unwrap_or_else(|_| "config/dev/auditor.toml".to_string());
        let content = std::fs::read_to_string(config_path)?;
        let cfg: Config = toml::from_str(&content)?;
        Ok(cfg)
    }
}
