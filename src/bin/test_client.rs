use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use jsonrpsee::core::client::ClientT; // Import the ClientT trait

#[derive(Debug, Deserialize, Serialize)]
struct Transaction {
    from: String,
    to: String,
    signature: String,
    transaction_data: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create the client
    let client = HttpClientBuilder::default().build("http://127.0.0.1:8000")?;

    // Create a Transaction instance
    let transaction = Transaction {
        from: "0xprazolgod".to_string(),
        to: "0xsequencer".to_string(),
        signature: "twinexyz".to_string(),
        transaction_data: "twine is the great".to_string(),
    };

    // Use rpc_params! to create params from the transaction
    let tx_params = rpc_params![
        transaction.from,
        transaction.to,
        transaction.signature,
        transaction.transaction_data
    ];

    // Submit a transaction
    let submit_response: Result<String, _> = client.request("submit_transaction", tx_params).await;
    match submit_response {
        Ok(response) => println!("Submit Transaction Response: {:?}", response),
        Err(err) => eprintln!("Failed to submit transaction: {:?}", err),
    }

    // Get mempool transactions
    let mempool_response: Result<Vec<Transaction>, _> = client.request("get_mempool", rpc_params![]).await;
    match mempool_response {
        Ok(transactions) => println!("Mempool Transactions: {:?}", transactions),
        Err(err) => eprintln!("Failed to get mempool transactions: {:?}", err),
    }

    Ok(())
}
