use ethers::types::TransactionRequest;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencerTransaction {
    pub tx: TransactionRequest,  // The actual transaction request
    pub timestamp: u64,          // Unix timestamp for ordering
}

impl SequencerTransaction {
    // Create a new transaction with the current timestamp
    pub fn new(tx: TransactionRequest) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        SequencerTransaction { tx, timestamp }
    }
}
