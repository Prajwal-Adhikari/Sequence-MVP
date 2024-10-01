mod mempool;
mod rpc;
mod transaction;

use mempool::Mempool;
use rpc::run_rpc_server;
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mempool = Arc::new(Mempool::new());

    println!("Starting the JSON-RPC server....");
    run_rpc_server(mempool).await?;

    Ok(())
}