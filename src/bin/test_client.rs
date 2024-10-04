use ethers::types::{TransactionRequest, H160};
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use jsonrpsee::rpc_params;
use anyhow::Result;
use tokio::time::{sleep, Duration};
use jsonrpsee::core::client::ClientT; // Import the ClientT trait to use the `request` method

#[tokio::main]
async fn main() -> Result<()> {
    // Create the client to communicate with the server
    let client = HttpClientBuilder::default().build("http://127.0.0.1:8000")?;

    // Loop to continuously send transactions every 2 seconds
    loop {
        // Simulate creating a transaction
        let from: H160 = "0x1234567890abcdef1234567890abcdef12345678"
            .parse()
            .expect("Invalid from address");
        let to: H160 = "0x1234567890abcdef1234567890abcdef12345679"
            .parse()
            .expect("Invalid to address");
        let signature = "YourTransactionSignature".to_string();
        let transaction_data = "YourTransactionData".to_string();

        // Create a TransactionRequest (using ethers-rs)
        let tx_request = TransactionRequest {
            from: Some(from.into()),  // Convert H160 to NameOrAddress
            to: Some(to.into()),      // Convert H160 to NameOrAddress
            gas: Some(21000.into()),  // Example gas limit
            value: Some(1000.into()), // Example value
            nonce: None,              // You can set this based on your logic
            ..Default::default()
        };

        // Prepare RPC params for submitting the transaction
        let tx_params = rpc_params![
            format!("{:?}", tx_request.from.unwrap()),
            format!("{:?}", tx_request.to.unwrap()),
            signature,
            transaction_data
        ];

        // Send the transaction to the server via the JSON-RPC endpoint
        let submit_response: Result<String, _> = client.request("submit_transaction", tx_params).await;
        match submit_response {
            Ok(response) => println!("Submit Transaction Response: {:?}", response),
            Err(err) => eprintln!("Failed to submit transaction: {:?}", err),
        }

        // Sleep for 2 seconds before sending the next transaction
        sleep(Duration::from_secs(2)).await;
    }
}
