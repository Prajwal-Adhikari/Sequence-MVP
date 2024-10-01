use crate::transaction::Transaction;
use std::sync::Mutex;

pub struct Mempool {
    pub transactions: Mutex<Vec<Transaction>>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {
            transactions: Mutex::new(Vec::new()),
        }
    }

    pub fn add_transaction(&self, tx:Transaction) {
        let mut txs = self.transactions.lock().unwrap();
        txs.push(tx);
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        let txs = self.transactions.lock().unwrap();
        txs.clone()
    }
}