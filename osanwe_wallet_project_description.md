
# Detailed Project Description: Osanwe Wallet CLI

## Overview
Osanwe Wallet CLI is a prototype that enables interaction with a decentralized microtransaction system without blockchain. It consists of several modules, each responsible for specific functionality. The project is written in Rust, ensuring high performance, memory safety, and ease of integration.

---

## Architecture

### **Modules**

1. **Wallet Module (`wallet.rs`)**
   - Manages the user's wallet.
   - Functions:
     - Generates public/private key pairs using Ed25519.
     - Saves and loads balances and transaction histories.
     - Generates wallet addresses.
   - Data is stored in JSON format in a local file `wallet.json`.

2. **Transaction Module (`transaction.rs`)**
   - Handles creation, signing, and verification of transactions.
   - Functions:
     - Creates digital receipts.
     - Signs transactions using Ed25519.
     - Validates transactions (balance, signature, timestamps).

3. **DHT Module (`dht.rs`)**
   - Implements interaction with the distributed hash table via `libp2p`.
   - Functions:
     - Adds records to the DHT.
     - Fetches transactions from the DHT for validation.
     - Synchronizes data between participants.

4. **CLI Module (`main.rs`)**
   - Provides the command-line interface.
   - Functions:
     - Commands for creating wallets, viewing balances, making transactions, syncing, and validation.

5. **Testing Module (`tests.rs`)**
   - Contains automated tests for:
     - Cryptography correctness using Ed25519.
     - Interaction with the DHT.
     - Balance and transaction logic.

---

## File Structure

```
DHT_Wallet_CLI/
├── Cargo.toml                  # Rust configuration file, project dependencies.
├── src/
│   ├── main.rs                 # CLI interface.
│   ├── wallet.rs               # Wallet logic.
│   ├── transaction.rs          # Transaction logic.
│   ├── dht.rs                  # Interaction with DHT via libp2p.
│   └── utils.rs                # Helper functions (hashing, validation).
├── tests/
│   ├── wallet_tests.rs         # Tests for wallet module.
│   ├── transaction_tests.rs    # Tests for transaction module.
│   ├── dht_tests.rs            # Tests for DHT module.
│   └── integration_tests.rs    # Integration tests.
├── wallet.json                 # Sample wallet state storage.
└── README.md                   # Project documentation.
```

---

## **Module Details**

### `wallet.rs`
- **Class Wallet**
  - `new(public_key: String, private_key: String)`: Creates a new wallet.
  - `load_from_file(path: &str)`: Loads a wallet from a JSON file.
  - `save_to_file(path: &str)`: Saves the wallet to a JSON file.
  - `update_balance(amount: i64)`: Updates the balance.

### `transaction.rs`
- **Class Transaction**
  - `new(sender: &str, recipient: &str, amount: u64)`: Creates a new transaction.
  - `sign(private_key: &str)`: Signs the transaction using Ed25519.
  - `verify_signature() -> bool`: Verifies the transaction's signature using the sender's public key.
  - `is_valid_balance(balance: u64) -> bool`: Checks if the balance is sufficient for the transaction.

### `dht.rs`
- **Class DHT**
  - `add_record(key: Vec<u8>, value: Vec<u8>)`: Adds a record to the DHT.
  - `get_record(key: Vec<u8>) -> Option<Vec<u8>>`: Retrieves a record from the DHT.
  - `sync()`: Synchronizes data between nodes.

### `main.rs`
- Implements CLI commands:
  - `create-wallet`: Creates a new wallet with realistic Ed25519 keys.
  - `show-wallet`: Displays wallet information including Ed25519 public key.
  - `send <recipient_public_key> <amount>`: Executes a transaction.
  - `sync`: Synchronizes with the network.
  - `validate`: Validates transactions.

---

## **Automated Testing**

### Test Setup
- Tests are located in the `tests/` directory.
- Standard Rust testing tools (`cargo test`) are used.
- Types of tests:
  1. **Unit Tests**:
     - Tests for each module's functions.
  2. **Integration Tests**:
     - Tests the interaction between modules (wallet, transactions, and DHT).

### Running Tests
```bash
cargo test
```
- Example of successful output:
```
running 10 tests
test wallet_tests::test_wallet_creation ... ok
test transaction_tests::test_transaction_signing ... ok
test dht_tests::test_sync ... ok
test integration_tests::test_full_workflow ... ok
```

---

## **Deployment Instructions**

1. **Download the Precompiled Binary**
   - Download the binary for your OS from the GitHub releases page.

2. **Make the Binary Executable**
   - Ensure the binary is executable:
     ```bash
     chmod +x dht-wallet
     ```

3. **Run the Program**
   - Use the binary directly to execute commands, e.g.:
     ```bash
     dht-wallet create-wallet
     ```

4. **Run Tests** (if building from source)
   - To verify the correctness of the code:
     ```bash
     cargo test
     ```

---

## **Potential Improvements**
1. **Add Docker for quick deployment.**
2. **Implement QR code support for address entry.**
3. **Integrate with mobile applications via FFI.**

This description ensures alignment with realistic cryptographic standards and the use of Ed25519 keys. Let me know if further adjustments are needed! 🚀
