use clap::{Parser, Subcommand};
use reqwest::Client;
use serde_json::Value;
use anyhow::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Verify a receipt file
    Verify {
        /// Path to receipt file (JSON)
        receipt_file: String,
        /// Optional API endpoint (defaults to http://localhost:3001/verify)
        #[arg(short, long, default_value = "http://localhost:3001/verify")]
        api: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Verify { receipt_file, api } => {
            let receipt_content = std::fs::read_to_string(receipt_file)?;
            let receipt_json: Value = serde_json::from_str(&receipt_content)?;
            let client = Client::new();
            let response = client.post(&api)
                .json(&receipt_json)
                .send()
                .await?;
            let result: Value = response.json().await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }
    Ok(())
}
