use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use crate::mempool::Mempool;
use crate::transaction::SequencerTransaction;
use anyhow::Result;
use reqwest::Client as HttpClient;

/// Batcher that picks up transactions from the mempool, batches them, and sends to zkVM.
pub struct Batcher {
    mempool: Arc<Mempool>,          // Shared access to the mempool
    zkvm_endpoint: String,          // The zkVM endpoint URL
}

impl Batcher {
    pub fn new(mempool: Arc<Mempool>, zkvm_endpoint: String) -> Self {
        Self {
            mempool,
            zkvm_endpoint,
        }
    }

    /// The batcher function that runs in the background.
    pub async fn run(self: Arc<Self>) -> Result<()> {
        let client = HttpClient::new();

        loop {
            // Sleep for a sequencing window (e.g., 10 seconds) before picking up transactions
            sleep(Duration::from_secs(10)).await;

            // Get transactions from the mempool
            let transactions = self.mempool.get_all_transactions();
            
            if !transactions.is_empty() {
                println!("Batcher: Found {} transactions to batch.", transactions.len());

                // Log the state of transactions being picked up by the batcher
                for tx in &transactions {
                    println!(
                        "Batcher: Picking up transaction from {:?} to {:?}, timestamp: {}",
                        tx.tx.from, tx.tx.to, tx.timestamp
                    );
                }

                // Send batch to the zkVM
                match self.send_batch_to_zkvm(&transactions, &client).await {
                    Ok(_) => {
                        println!("Batcher: Successfully sent batch to zkVM.");
                        // Clear the mempool after successfully sending the batch
                        self.mempool.clear();  // Clear the mempool after sending the batch
                    }
                    Err(e) => {
                        eprintln!("Batcher: Failed to send batch to zkVM: {:?}", e);
                    }
                }
            } else {
                println!("Batcher: No transactions found for batching.");
            }
        }
    }

    /// Sends the batch of transactions to the zkVM endpoint.
    async fn send_batch_to_zkvm(&self, transactions: &[SequencerTransaction], client: &HttpClient) -> Result<()> {
        let zkvm_url = &self.zkvm_endpoint;

        // Serialize the batch of transactions to JSON
        let batch_json = serde_json::to_string(transactions)?;

        // Send a POST request to the zkVM endpoint
        let response = client.post(zkvm_url)
            .header("Content-Type", "application/json")
            .body(batch_json)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Failed to send batch to zkVM: HTTP Status {}",
                response.status()
            ))
        }
    }
}
