mod mempool;
mod rpc;
mod transaction;
mod batcher;

use mempool::Mempool;
use rpc::run_rpc_server;
use batcher::Batcher;
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mempool = Arc::new(Mempool::new());

    // Start the RPC server to handle transactions, provide the address for the server to bind to
    println!("Starting the JSON-RPC server....");
    let rpc_address = "127.0.0.1:8000";  // Add the binding address for the RPC server
    let rpc_handle = tokio::spawn(run_rpc_server(Arc::clone(&mempool), rpc_address));

    // Start the Batcher to pick up transactions and send them to zkVM
    let zkvm_endpoint = "http://localhost:5000/zkvm".to_string();  // Replace with your actual zkVM endpoint
    let batcher = Arc::new(Batcher::new(Arc::clone(&mempool), zkvm_endpoint));  // Wrap in Arc
    
    // Move the batcher into the task
    let batcher_handle = {
        let batcher_clone = Arc::clone(&batcher);  // Clone Arc for use in the task
        tokio::spawn(async move {
            batcher_clone.run().await; // Call run on the cloned Arc
        })
    };

    // Wait for both the RPC server and the batcher to finish
    rpc_handle.await??;
    batcher_handle.await?;

    Ok(())
}
