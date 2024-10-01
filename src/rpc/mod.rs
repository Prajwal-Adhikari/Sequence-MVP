use crate::mempool::Mempool;
use crate::transaction::Transaction;

use jsonrpsee::server::{RpcModule, ServerBuilder};
use std::net::SocketAddr;
use std::sync::Arc;

pub async fn run_rpc_server(mempool: Arc<Mempool>) -> anyhow::Result<()> {
    let server_addr = "127.0.0.1:8000".parse::<SocketAddr>()?;

    // Use ServerBuilder to build the server
    let server = ServerBuilder::default().build(server_addr).await?;

        let mut module = RpcModule::new(mempool.clone());

    // Method to submit a transaction
    // module.register_method("submit_transaction", |params, mempool| {
    //     let tx: Transaction = params.parse()?;
    //     mempool.add_transaction(tx.clone());
    //     Ok(format!("Transaction from {} to {} submitted", tx.from, tx.to))
    // })?;

    module.register_method("submit_transaction", |params, mempool| {
        let (from, to, signature, transaction_data): (String, String, String, String) = params.parse()?;
        
        let tx = Transaction {
            from,
            to,
            signature,
            transaction_data,
        };
        
        mempool.add_transaction(tx.clone());
        Ok(format!("Transaction from {} to {} submitted", tx.from, tx.to))
    })?;
    
    // Method to retrieve transactions
    module.register_method("get_mempool", |_, mempool| {
        let txs = mempool.get_transactions();
        Ok(txs)
    })?;


    let handle = server.start(module)?;


    handle.stopped().await;
    Ok(())
}
