use ethers::types::{TransactionRequest, H160, NameOrAddress};
// use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder}; 
use jsonrpsee::rpc_params;
use anyhow::Result;
use jsonrpsee::core::client::ClientT; // Import the ClientT trait
use std::sync::Arc;

async fn get_mempool_length(client: Arc<HttpClient>) -> Result<u64, jsonrpsee::core::Error> {
    // Call the get_mempool_length method with an empty tuple for no parameters
    let length: u64 = client.request("get_mempool_length").await?;
    Ok(length)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create the client
    let client = HttpClientBuilder::default().build("http://127.0.0.1:8000")?;

    // Define transaction parameters
    let from: H160 = "0x1234567890abcdef1234567890abcdef12345678".parse().expect("Invalid from address");
    let to: H160 = "0x1234567890abcdef1234567890abcdef12345679".parse().expect("Invalid to address");
    let signature = "YourTransactionSignature".to_string();
    let transaction_data = "YourTransactionData".to_string();

    // Create a TransactionRequest
    let tx_request = TransactionRequest {
        from: Some(from.into()),  // Convert H160 to NameOrAddress
        to: Some(to.into()),      // Convert H160 to NameOrAddress
        gas: Some(21000.into()),  // Example gas limit
        value: Some(1000.into()), // Example value
        nonce: None,              // You can set this based on your logic
        ..Default::default()
    };

    // Use rpc_params! to create params from the TransactionRequest
    let tx_params = rpc_params![
        format!("{:?}", tx_request.from.unwrap()), // Convert H160 to string
        format!("{:?}", tx_request.to.unwrap()),   // Convert H160 to string
        signature,
        transaction_data
    ];

    // Submit a transaction
    let submit_response: Result<String, _> = client.request("submit_transaction", tx_params).await;
    match submit_response {
        Ok(response) => println!("Submit Transaction Response: {:?}", response),
        Err(err) => eprintln!("Failed to submit transaction: {:?}", err),
    }

    // Get mempool transactions
    let mempool_response: Result<Vec<TransactionRequest>, _> = client.request("get_mempool", rpc_params![]).await;
    match mempool_response {
        Ok(transactions) => println!("Mempool Transactions: {:?}", transactions),
        Err(err) => eprintln!("Failed to get mempool transactions: {:?}", err),
    }

    match get_mempool_length(client.clone()).await {
        Ok(length) => println!("Current length of the mempool: {}", length),
        Err(e) => eprintln!("Failed to retrieve mempool length: {:?}", e),
    }


    Ok(())
}


