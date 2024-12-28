
# User Guide: Osanwe Wallet CLI

## Overview
The Osanwe Wallet CLI is a command-line application that allows users to interact with a decentralized microtransaction system. This guide provides step-by-step instructions on how to use the wallet, including how to create a wallet, view its details, make transactions, and synchronize with the DHT network.

---

## Prerequisites
1. Download the precompiled binary for your operating system from the GitHub releases page.
2. Place the binary in a directory included in your system's PATH or navigate to the directory containing the binary.

---

## Starting the Application
Run the application by executing the binary:
```bash
dht-wallet <command>
```

Replace `<command>` with any of the commands described below.

---

## Commands and Usage

### 1. **Create a Wallet**
Generates a new wallet with an Ed25519 public/private key pair and an initial balance.

- Command:
  ```bash
  dht-wallet create-wallet
  ```
- Output example:
  ```
  Wallet created successfully!
  Public Key: d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a
  Balance: 1000 units
  ```
- The wallet information is saved locally in `wallet.json`.

---

### 2. **View Wallet Information**
Displays the public address (Ed25519 public key), current balance, and recent transactions.

- Command:
  ```bash
  dht-wallet show-wallet
  ```
- Output example:
  ```
  Wallet Information:
  Public Key: d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a
  Balance: 950 units
  Recent Transactions:
    - To: af021a68f707511ad75a980182b10ab7d54bfed3c964073a0ee172f3daa62325, Amount: 50, Timestamp: 2024-12-28T10:00:00Z
  ```

---

### 3. **Make a Transaction**
Sends funds to another wallet by creating a digital receipt signed with your private key.

- Command:
  ```bash
  dht-wallet send <recipient_public_key> <amount>
  ```
- Example:
  ```bash
  dht-wallet send af021a68f707511ad75a980182b10ab7d54bfed3c964073a0ee172f3daa62325 50
  ```
- Output example:
  ```
  Transaction successful!
  Sent 50 units to af021a68f707511ad75a980182b10ab7d54bfed3c964073a0ee172f3daa62325.
  New Balance: 900 units.
  ```

---

### 4. **Validate Transactions**
Checks for any conflicts or invalid transactions.

- Command:
  ```bash
  dht-wallet validate
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
  dht-wallet sync
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
   dht-wallet create-wallet
   ```

2. **View wallet details:**
   ```bash
   dht-wallet show-wallet
   ```

3. **Send funds to another wallet:**
   ```bash
   dht-wallet send af021a68f707511ad75a980182b10ab7d54bfed3c964073a0ee172f3daa62325 50
   ```

4. **Validate transactions:**
   ```bash
   dht-wallet validate
   ```

5. **Synchronize with the network:**
   ```bash
   dht-wallet sync
   ```

---

## Additional Notes
- All wallet data is stored locally in `wallet.json`. Ensure the file is secure to protect your private key.
- If synchronization with the DHT fails, ensure that your network connection is stable.

---

## Troubleshooting
### Problem: The binary cannot be executed
- Solution: Ensure the binary is executable by running:
  ```bash
  chmod +x dht-wallet
  ```

### Problem: Wallet file missing or corrupted
- Solution: Recreate the wallet by running:
  ```bash
  dht-wallet create-wallet
  ```

### Problem: Unable to synchronize with the DHT
- Solution: Check your network connection and ensure no firewall blocks the application.

---

## Next Steps
- Explore additional features like integrating with a mobile application or enhancing privacy using zero-knowledge proofs.

This guide ensures that users can fully utilize the Osanwe Wallet CLI with minimal setup and effort.
