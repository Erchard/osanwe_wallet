
# User Guide: Osanwe Wallet CLI

## Overview
The Osanwe Wallet CLI is a command-line application that allows users to interact with a decentralized microtransaction system. This guide provides step-by-step instructions on how to use the wallet, including how to create a wallet, view its details, make transactions, and synchronize with the DHT network.

---

## Prerequisites
1. Install Rust: Follow the instructions at [Rust official site](https://rustup.rs/).
2. Clone the repository:
   ```bash
   git clone https://github.com/your-username/DHT_Wallet_CLI.git
   cd DHT_Wallet_CLI
   ```
3. Build the project:
   ```bash
   cargo build
   ```

---

## Starting the Application
Run the application with:
```bash
cargo run -- <command>
```

Replace `<command>` with any of the commands described below.

---

## Commands and Usage

### 1. **Create a Wallet**
Generates a new wallet with a public/private key pair and an initial balance.

- Command:
  ```bash
  cargo run -- create-wallet
  ```
- Output example:
  ```
  Wallet created successfully!
  Public Key: abc123
  Balance: 1000 units
  ```
- The wallet information is saved locally in `wallet.json`.

---

### 2. **View Wallet Information**
Displays the public address, current balance, and recent transactions.

- Command:
  ```bash
  cargo run -- show-wallet
  ```
- Output example:
  ```
  Wallet Information:
  Public Key: abc123
  Balance: 950 units
  Recent Transactions:
    - To: def456, Amount: 50, Timestamp: 2024-12-28T10:00:00Z
  ```

---

### 3. **Make a Transaction**
Sends funds to another wallet by creating a digital receipt signed with your private key.

- Command:
  ```bash
  cargo run -- send <recipient_address> <amount>
  ```
- Example:
  ```bash
  cargo run -- send def456 50
  ```
- Output example:
  ```
  Transaction successful!
  Sent 50 units to def456.
  New Balance: 900 units.
  ```

---

### 4. **Validate Transactions**
Checks for any conflicts or invalid transactions.

- Command:
  ```bash
  cargo run -- validate
  ```
- Output example:
  ```
  Validation complete. No conflicts detected.
  ```

---

### 5. **Synchronize with DHT**
Retrieves the latest transaction data from the DHT network.

- Command:
  ```bash
  cargo run -- sync
  ```
- Output example:
  ```
  Synchronization complete. Latest transactions fetched.
  ```

---

## Workflow Example
Here’s an example workflow for using the wallet:

1. **Create a wallet:**
   ```bash
   cargo run -- create-wallet
   ```

2. **View wallet details:**
   ```bash
   cargo run -- show-wallet
   ```

3. **Send funds to another wallet:**
   ```bash
   cargo run -- send def456 50
   ```

4. **Validate transactions:**
   ```bash
   cargo run -- validate
   ```

5. **Synchronize with the network:**
   ```bash
   cargo run -- sync
   ```

---

## Additional Notes
- All wallet data is stored locally in `wallet.json`. Ensure the file is secure to protect your private key.
- If synchronization with the DHT fails, ensure that your network connection is stable.

---

## Troubleshooting
### Problem: `cargo` command not found
- Solution: Ensure Rust is installed by running:
  ```bash
  rustup --version
  ```

### Problem: Wallet file missing or corrupted
- Solution: Recreate the wallet by running:
  ```bash
  cargo run -- create-wallet
  ```

### Problem: Unable to synchronize with the DHT
- Solution: Check your network connection and ensure no firewall blocks the application.

---

## Next Steps
- Explore additional features like integrating with a mobile application or enhancing privacy using zero-knowledge proofs.

This guide ensures that users can fully utilize the Osanwe Wallet CLI with minimal setup and effort.
