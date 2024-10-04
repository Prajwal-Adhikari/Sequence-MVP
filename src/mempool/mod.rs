use std::collections::VecDeque;
use std::sync::Mutex;
use crate::transaction::SequencerTransaction;

/// Mempool struct that holds transactions
pub struct Mempool {
    pub transactions: Mutex<VecDeque<SequencerTransaction>>,  // Use Mutex for thread safety
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            transactions: Mutex::new(VecDeque::new()),  // Initialize with a VecDeque inside the Mutex
        }
    }

    /// Adds a transaction to the mempool
    pub fn add_transaction(&self, tx: SequencerTransaction) {
        let mut transactions = self.transactions.lock().unwrap();  // Lock the Mutex before modifying
        transactions.push_back(tx);
    }

    /// Retrieves and clones all transactions from the mempool
    pub fn get_all_transactions(&self) -> Vec<SequencerTransaction> {
        let transactions = self.transactions.lock().unwrap();  // Lock the Mutex before reading
        transactions.clone().into_iter().collect()
    }

    /// Clears the mempool after batch is processed
    pub fn clear(&self) {
        let mut transactions = self.transactions.lock().unwrap();  // Lock the Mutex before modifying
        transactions.clear();
    }

    /// Returns the length of the mempool
    pub fn len(&self) -> usize {
        let transactions = self.transactions.lock().unwrap();  // Lock the Mutex before reading
        transactions.len()
    }
}
