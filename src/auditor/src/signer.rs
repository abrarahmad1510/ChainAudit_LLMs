use anyhow::Result;
use ed25519_dalek::{SigningKey, Signer as EdSigner};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use crate::config::SigstoreConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub leaf_hash: String,
    pub leaf_index: i64,
    pub root_hash: String,
    pub inclusion_proof: Vec<String>,
    pub timestamp: String,
    pub metadata: serde_json::Value,
    pub signature: String,
    pub public_key: String,
}

pub struct Signer {
    signing_key: SigningKey,
    public_key: Vec<u8>,
}

impl Signer {
    pub async fn new(_cfg: &SigstoreConfig) -> Result<Self> {
        let signing_key = SigningKey::generate(&mut OsRng);
        let public_key = signing_key.verifying_key().to_bytes().to_vec();
        Ok(Self { signing_key, public_key })
    }

    pub async fn sign_receipt(
        &self,
        leaf_hash: &[u8],
        leaf_index: i64,
        root_hash: &[u8],
        inclusion_proof: &[Vec<u8>],
        metadata: &[u8],
    ) -> Result<String> {
        let metadata: serde_json::Value = serde_json::from_slice(metadata)?;
        let proof_hex: Vec<String> = inclusion_proof.iter()
            .map(|h| hex::encode(h))
            .collect();
        let timestamp = chrono::Utc::now().to_rfc3339();

        let canonical_string = format!(
            "{}:{}:{}:{}",
            hex::encode(leaf_hash),
            leaf_index,
            hex::encode(root_hash),
            timestamp
        );
        let signature = self.signing_key.sign(canonical_string.as_bytes());

        let receipt = Receipt {
            leaf_hash: hex::encode(leaf_hash),
            leaf_index,
            root_hash: hex::encode(root_hash),
            inclusion_proof: proof_hex,
            timestamp: timestamp.clone(),
            metadata,
            signature: BASE64.encode(signature.to_bytes()),
            public_key: BASE64.encode(&self.public_key),
        };

        let receipt_json = serde_json::to_string(&receipt)?;
        Ok(receipt_json)
    }
}
