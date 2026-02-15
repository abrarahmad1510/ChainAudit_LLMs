mod config;
mod server;
mod signer;
mod storage;
mod kafka;
mod trillian;

pub mod auditor {
    include!(concat!(env!("OUT_DIR"), "/auditor.rs"));
}

pub mod google {
    pub mod rpc {
        include!(concat!(env!("OUT_DIR"), "/google.rpc.rs"));
    }
}

use anyhow::Result;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cfg = config::Config::from_env()?;
    server::run(cfg).await?;
    Ok(())
}
