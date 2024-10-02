use crate::transaction::SequencerTransaction;
use std::sync::Mutex;

pub struct Mempool {
    pub transactions: Mutex<Vec<SequencerTransaction>>,  // Store list of SequencerTransactions
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {
            transactions: Mutex::new(Vec::new()),  // Initialize with an empty Vec
        }
    }

    // Add a new transaction to the mempool
    pub fn add_transaction(&self, tx: SequencerTransaction) {
        let mut txs = self.transactions.lock().unwrap();
        txs.push(tx);
    }

    pub fn get_length(&self) -> u64 {
        self.transactions.lock().unwrap().len() as u64 // Replace with your actual method to get the length
    }
    // Retrieve all transactions in the mempool
    pub fn get_transactions(&self) -> Vec<SequencerTransaction> {
        let txs = self.transactions.lock().unwrap();
        txs.clone()
    }

    // Retrieve transactions ordered by timestamp
    pub fn get_ordered_transactions(&self) -> Vec<SequencerTransaction> {
        let mut txs = self.transactions.lock().unwrap().clone();
        txs.sort_by_key(|tx| tx.timestamp);  // Sort by timestamp (oldest first)
        txs
    }
}