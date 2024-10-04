use crate::mempool::Mempool;
use crate::transaction::SequencerTransaction;
use ethers::types::{TransactionRequest, NameOrAddress};
use jsonrpsee::server::{RpcModule, ServerBuilder};
use jsonrpsee::types::params;
use reqwest::Client as HttpClient; // Used for sending requests to zkVM endpoint
use std::net::SocketAddr;
use std::sync::Arc;
use serde_json::json;
use tokio::time::{sleep, Duration};
use tokio::task;

pub async fn run_rpc_server(mempool: Arc<Mempool>, zkvm_url: &str) -> anyhow::Result<()> {
    let server_addr = "127.0.0.1:8000".parse::<SocketAddr>()?;
    let server = ServerBuilder::default().build(server_addr).await?;

    let mut module = RpcModule::new(mempool.clone());

    // Method to submit a transaction
    module.register_method("submit_transaction", |params, mempool| {
        let (from, to, signature, transaction_data): (String, String, String, String) = params.parse()?;

        // Create a TransactionRequest from the provided data
        let tx_request = TransactionRequest {
            from: Some(from.parse().expect("Invalid from address")),
            to: Some(to.parse().expect("Invalid to address")),
            gas: Some(21000.into()), // Example gas limit
            value: Some(1000.into()), // Example value
            ..Default::default()
        };

        // Create a SequencerTransaction with a timestamp
        let sequencer_tx = SequencerTransaction::new(tx_request);

        // Add the transaction to the mempool
        mempool.add_transaction(sequencer_tx.clone());

        // Log transaction state: Added to mempool
        println!(
            "Transaction added to mempool: from {} to {} with signature '{}'",
            from, to, signature
        );

        Ok(format!(
            "Transaction from {} to {} with signature '{}' submitted", 
            from, to, signature
        ))
    })?;

    // Start the background batcher task
    let zkvm_url = zkvm_url.to_string();
    task::spawn(batch_process(mempool.clone(), zkvm_url));

    // Start the RPC server
    let handle = server.start(module)?;

    handle.stopped().await;
    Ok(())
}

// Batcher function to pick up transactions and send them to zkVM endpoint
async fn batch_process(mempool: Arc<Mempool>, zkvm_url: String) {
    let http_client = HttpClient::new();

    loop {
        // Simulate batching every 10 seconds (you can adjust the timing as needed)
        sleep(Duration::from_secs(10)).await;

        let transactions = mempool.get_all_transactions();
        if !transactions.is_empty() {
            // Log transaction state: Picked up by batcher
            println!("Batcher picked up {} transactions", transactions.len());

            // Send batch to zkVM endpoint
            let response = http_client.post(&zkvm_url)
                .json(&transactions)
                .send()
                .await;

            match response {
                Ok(res) => {
                    println!("Batch sent to zkVM, response: {:?}", res);
                    // Log transaction state: Sent to zkVM
                },
                Err(err) => {
                    eprintln!("Failed to send batch to zkVM: {:?}", err);
                }
            }
        } else {
            println!("No transactions to batch at this time.");
        }
    }
}
