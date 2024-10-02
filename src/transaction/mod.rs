use ethers::types::TransactionRequest;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencerTransaction {
    pub tx: TransactionRequest,  // The actual transaction from ethers
    pub timestamp: u64,          // Unix timestamp for ordering
}

impl SequencerTransaction {
    pub fn new(tx: TransactionRequest) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();  // Convert to seconds since UNIX epoch

        SequencerTransaction {
            tx,
            timestamp,
        }
    }
}

