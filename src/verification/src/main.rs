mod verify;

use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use anyhow::Result;
use tracing::info;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;

// Receipt struct matching the one in signer
#[derive(Debug, Deserialize)]
struct Receipt {
    leaf_hash: String,
    leaf_index: i64,
    root_hash: String,
    inclusion_proof: Vec<String>,
    timestamp: String,
    metadata: serde_json::Value,
    signature: String,
    public_key: String,
}

#[derive(Debug, Deserialize)]
struct VerifyRequest {
    receipt: String, // JSON string of Receipt (or base64 encoded)
}

#[derive(Debug, Serialize)]
struct VerifyResponse {
    valid: bool,
    message: String,
}

struct AppState {}

async fn verify_handler(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<VerifyRequest>,
) -> Json<VerifyResponse> {
    // Try to parse as JSON directly (if not base64 encoded)
    let receipt: Receipt = match serde_json::from_str(&req.receipt) {
        Ok(r) => r,
        Err(_) => {
            // If that fails, try base64 decode then JSON parse
            match BASE64.decode(&req.receipt) {
                Ok(bytes) => {
                    match serde_json::from_slice(&bytes) {
                        Ok(r) => r,
                        Err(e) => {
                            return Json(VerifyResponse {
                                valid: false,
                                message: format!("Invalid receipt JSON after base64: {}", e),
                            });
                        }
                    }
                }
                Err(e) => {
                    return Json(VerifyResponse {
                        valid: false,
                        message: format!("Invalid base64: {}", e),
                    });
                }
            }
        }
    };

    // Perform verification
    match verify::verify_receipt(&receipt).await {
        Ok(valid) => {
            if valid {
                Json(VerifyResponse {
                    valid: true,
                    message: "Receipt is valid".to_string(),
                })
            } else {
                Json(VerifyResponse {
                    valid: false,
                    message: "Receipt verification failed".to_string(),
                })
            }
        }
        Err(e) => {
            Json(VerifyResponse {
                valid: false,
                message: format!("Verification error: {}", e),
            })
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let state = Arc::new(AppState {});
    let app = Router::new()
        .route("/verify", post(verify_handler))
        .with_state(state);
    let addr = "0.0.0.0:3001".parse()?;
    info!("Verification API listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
