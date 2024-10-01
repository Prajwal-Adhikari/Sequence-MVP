# Rollup Sequencer MVP

## Overview

This project is a Minimum Viable Product (MVP) for a rollup sequencer, implementing a JSON-RPC server using jsonrpsee to receive transactions from users. The sequencer maintains a mempool to store and order transactions, which can then be executed in batches by a zkVM.

## Features

- **JSON-RPC Server**: Receives transactions via HTTP.
- **Mempool**: Stores and orders transactions.
- **Transaction Handling**: Supports submitting and retrieving transactions.



### Components

1. **main.rs**
   - This is the entry point of the application where the JSON-RPC server is initialized. It sets up the mempool and starts the RPC server, allowing it to listen for incoming transactions.

2. **mempool.rs**
   - This module defines the `Mempool` structure, which manages a collection of transactions. It provides methods to add transactions and retrieve the list of stored transactions. The transactions are protected using a `Mutex` to ensure thread safety.

3. **rpc.rs**
   - This module implements the JSON-RPC server, handling incoming requests for submitting and retrieving transactions. It defines RPC methods such as `submit_transaction` for adding new transactions to the mempool and `get_mempool` for retrieving the current list of transactions.

4. **transaction.rs**
   - This module defines the `Transaction` structure, which represents a transaction in the mempool. It includes fields such as the sender address, recipient address, signature, and transaction data. The structure derives serialization and deserialization capabilities using the `serde` library.

## Dependencies

Ensure that the following dependencies are included in your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
jsonrpsee = { version = "0.16.2",  features = ["server","http-client"]}
jsonrpsee-core = "0.16.2"
serde = { version = "1.0", features = ["derive"] }
futures = "0.3"
anyhow = "1.0"
```

## Play around
To run the Rollup Sequencer MVP, follow these steps:

1. **Clone the Repository**:
   If you haven't already, clone the repository to your local machine:
   ```bash
   git clone <repository-url>
   cd <repository-directory>

2. **Run the sequencer_mvp server**:
    ```bash
    cargo run --bin sequencer_mvp 
    ```

3. **With the server running, in a separate terminal, run the test_client client**:
    ```bash
    cargo run --bin test_client
    ```