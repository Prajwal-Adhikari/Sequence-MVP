// use crate::mempool::Mempool;
// use crate::transaction::SequencerTransaction;
// use ethers::types::TransactionRequest;
// use jsonrpsee::server::{RpcModule, ServerBuilder};
// use std::net::SocketAddr;
// use std::sync::Arc;

// pub async fn run_rpc_server(mempool: Arc<Mempool>) -> anyhow::Result<()> {
//     let server_addr = "127.0.0.1:8000".parse::<SocketAddr>()?;

//     // Use ServerBuilder to build the server
//     let server = ServerBuilder::default().build(server_addr).await?;

//     let mut module = RpcModule::new(mempool.clone());

//     module.register_method("submit_transaction", |params, mempool| {
//         let (from, to, signature, transaction_data): (String, String, String, String) = params.parse()?;
    
//         // Create a TransactionRequest from the provided data
//         let tx_request = TransactionRequest {
//             from: Some(from.parse().expect("Invalid from address")),
//             to: Some(to.parse().expect("Invalid to address")),
//             gas: Some(21000.into()), // Example gas limit
//             value: Some(1000.into()), // Example value
//             nonce: None,              // You can set this based on your logic
//             ..Default::default()
//         };
    
//         // Create a SequencerTransaction
//         let sequencer_tx = SequencerTransaction::new(tx_request);
        
//         // Add transaction to the mempool
//         mempool.add_transaction(sequencer_tx.clone());
//         Ok(format!("Transaction from {:?} to {:?} with signature '{}' and data '{}' submitted", 
//             sequencer_tx.tx.from.unwrap(), sequencer_tx.tx.to.unwrap(), signature, transaction_data))
//     })?;


    
//         // Create a SequencerTransaction
//         let sequencer_tx = SequencerTransaction::new(tx_request);
        
//         // Add transaction to the mempool
//         mempool.add_transaction(sequencer_tx.clone());
        
//         // Convert H160 to string for display
//         let from_address = sequencer_tx.tx.from.unwrap().to_string();
//         let to_address = sequencer_tx.tx.to.unwrap().to_string();
        
//         Ok(format!(
//             "Transaction from {} to {} with signature '{}' and data '{}' submitted", 
//             from_address, to_address, signature, transaction_data
//         ))
//     })?;
    
    
//     // Method to retrieve transactions
//     module.register_method("get_mempool", |_, mempool| {
//         let txs = mempool.get_transactions();
//         Ok(txs)
//     })?;

//     // Start the RPC server
//     let handle = server.start(module)?;

//     handle.stopped().await;
//     Ok(())
// }

use crate::mempool::Mempool;
use crate::transaction::SequencerTransaction;
use ethers::types::{TransactionRequest, NameOrAddress, H160};
use jsonrpsee::server::{RpcModule, ServerBuilder};
use std::net::SocketAddr;
use std::sync::Arc;
use serde_json::json;

pub async fn run_rpc_server(mempool: Arc<Mempool>) -> anyhow::Result<()> {
    let server_addr = "127.0.0.1:8000".parse::<SocketAddr>()?;

    // Use ServerBuilder to build the server
    let server = ServerBuilder::default().build(server_addr).await?;

    let mut module = RpcModule::new(mempool.clone());

    module.register_method("submit_transaction", |params, mempool| {
        let (from, to, signature, transaction_data): (String, String, String, String) = params.parse()?;
    
        // Create a TransactionRequest from the provided data
        let tx_request = TransactionRequest {
            from: Some(from.parse().expect("Invalid from address")),
            to: Some(to.parse().expect("Invalid to address")),
            gas: Some(21000.into()), // Example gas limit
            value: Some(1000.into()), // Example value
            nonce: None,              // You can set this based on your logic
            ..Default::default()
        };
    
        // Create a SequencerTransaction
        let sequencer_tx = SequencerTransaction::new(tx_request);
        
        // Add transaction to the mempool
        mempool.add_transaction(sequencer_tx.clone());


        Ok(format!(
            "Transaction from {} to {} with signature '{}' and data '{}' submitted", 
            from,to,signature,transaction_data
        ))
    })?;
    
    // Method to retrieve transactions
// Method to retrieve transactions
// module.register_method("get_mempool", |_, mempool| {
//     let txs = mempool.get_transactions();

//     let serialized_txs: Vec<serde_json::Value> = txs.into_iter().map(|tx| {
//         let from_address = match tx.tx.from {
//             Some(NameOrAddress::Address(addr)) => addr.to_string(),
//             Some(NameOrAddress::Name(name)) => name,
//             None => "None".to_string(),
//         };

//         let to_address = match tx.tx.to {
//             Some(NameOrAddress::Address(addr)) => addr.to_string(),
//             Some(NameOrAddress::Name(name)) => name,
//             None => "None".to_string(),
//         };

//         json!({
//             "from": from_address,  // Directly use the converted string
//             "to": to_address,      // Directly use the converted string
//             "gas": tx.tx.gas,
//             "value": tx.tx.value,
//             "timestamp": tx.timestamp,
//         })
//     }).collect();

//     Ok(serialized_txs)
// })?;

// Method to retrieve transactions
module.register_method("get_mempool", |_, mempool| {
    let txs = mempool.get_transactions();

    let serialized_txs: Vec<serde_json::Value> = txs.into_iter().map(|tx| {
        // Handle Option<H160> directly
        let from_address = match tx.tx.from {
            Some(addr) => addr.to_string(), // Directly convert H160 to string
            None => "None".to_string(),
        };

        let to_address = match tx.tx.to {
            Some(NameOrAddress::Address(addr)) => addr.to_string(),
            Some(NameOrAddress::Name(name)) => name,
            None => "None".to_string(),
        };

        json!({
            "from": from_address,  // Use the converted string
            "to": to_address,      // Use the converted string
            "gas": tx.tx.gas,
            "value": tx.tx.value,
            "timestamp": tx.timestamp,
        })
    }).collect();

    Ok(serialized_txs)
})?;
// Method to retrieve the length of the mempool
module.register_method("get_mempool_length", |_, mempool| {
    let length = mempool.get_length(); // Assuming you have a `get_length` method in `Mempool`
    Ok(length)
})?;


    // Start the RPC server
    let handle = server.start(module)?;

    handle.stopped().await;
    Ok(())
}
