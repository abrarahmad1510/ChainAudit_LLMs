use anyhow::{anyhow, Result};
use serde_json::Value;
use blake3::Hash;
use ed25519_dalek::{VerifyingKey, Signature, Verifier};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use hex;

#[derive(Debug)]
struct ParsedReceipt {
    leaf_hash: Vec<u8>,
    leaf_index: i64,
    root_hash: Vec<u8>,
    inclusion_proof: Vec<Vec<u8>>,
    timestamp: String,
    metadata: Value,
    signature: Vec<u8>,
    public_key: Vec<u8>,
}

impl ParsedReceipt {
    fn from_json(receipt: &super::Receipt) -> Result<Self> {
        Ok(ParsedReceipt {
            leaf_hash: hex::decode(&receipt.leaf_hash)?,
            leaf_index: receipt.leaf_index,
            root_hash: hex::decode(&receipt.root_hash)?,
            inclusion_proof: receipt.inclusion_proof.iter()
                .map(|h| hex::decode(h))
                .collect::<Result<Vec<Vec<u8>>, hex::FromHexError>>()?,
            timestamp: receipt.timestamp.clone(),
            metadata: receipt.metadata.clone(),
            signature: BASE64.decode(&receipt.signature)?,
            public_key: BASE64.decode(&receipt.public_key)?,
        })
    }
}

pub async fn verify_receipt(receipt_json: &super::Receipt) -> Result<bool> {
    let receipt = ParsedReceipt::from_json(receipt_json)?;

    // 1. Hash recomputation
    let recomputed_leaf = recompute_leaf_hash(&receipt.metadata)?;
    if recomputed_leaf.as_bytes() != receipt.leaf_hash.as_slice() {
        return Ok(false);
    }

    // 2. Merkle inclusion proof verification
    if !verify_inclusion_proof(
        &receipt.leaf_hash,
        &receipt.root_hash,
        &receipt.inclusion_proof,
    )? {
        return Ok(false);
    }

    // 3. Signature verification
    if !verify_signature(
        &receipt.leaf_hash,
        receipt.leaf_index,
        &receipt.root_hash,
        &receipt.timestamp,
        &receipt.signature,
        &receipt.public_key,
    )? {
        return Ok(false);
    }

    Ok(true)
}

fn recompute_leaf_hash(context: &Value) -> Result<Hash> {
    let canonical = serde_json::to_vec(context)?;
    Ok(blake3::hash(&canonical))
}

fn verify_inclusion_proof(
    leaf_hash: &[u8],
    root_hash: &[u8],
    proof: &[Vec<u8>],
) -> Result<bool> {
    let mut current = leaf_hash.to_vec();
    for sibling in proof {
        let combined = [current.as_slice(), sibling.as_slice()].concat();
        current = blake3::hash(&combined).as_bytes().to_vec();
    }
    Ok(current == root_hash)
}

fn verify_signature(
    leaf_hash: &[u8],
    leaf_index: i64,
    root_hash: &[u8],
    timestamp: &str,
    signature: &[u8],
    public_key: &[u8],
) -> Result<bool> {
    // Reconstruct the canonical string that was signed
    let canonical_string = format!(
        "{}:{}:{}:{}",
        hex::encode(leaf_hash),
        leaf_index,
        hex::encode(root_hash),
        timestamp
    );

    // Convert public key and signature
    let verifying_key = VerifyingKey::from_bytes(public_key.try_into().map_err(|_| anyhow!("Invalid public key length"))?)?;
    let sig = Signature::from_bytes(signature.try_into().map_err(|_| anyhow!("Invalid signature length"))?);

    // Verify
    Ok(verifying_key.verify(canonical_string.as_bytes(), &sig).is_ok())
}
