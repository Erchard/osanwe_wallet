
# Technical Specification: CLI Client for Osanwe Wallet

## Goal
Develop a CLI client to demonstrate the core functionality of the Osanwe Wallet. The prototype should enable wallet creation, transaction handling, DHT interaction, and conflict validation.

---

## Functional Requirements

### 1. CLI Core Features
1. **Wallet Creation**
   - Users can create a new wallet.
   - Information to be stored:
     - Public and private keys (generated locally).
     - Initial balance (manually assigned for testing purposes).
   - Command: `create-wallet`

2. **View Wallet Information**
   - Displays public address, current balance, and recent transactions.
   - Command: `show-wallet`

3. **Transaction Execution**
   - Generates a digital receipt for transferring funds to another participant.
   - User inputs:
     - Recipient address.
     - Transfer amount.
   - Result:
     - A transaction is generated, signed with the private key.
     - The transaction is broadcasted to the DHT network.
   - Command: `send <recipient_address> <amount>`

4. **Transaction Validation**
   - Validates transactions for conflicts and double-spending.
   - Command: `validate`

5. **DHT Synchronization**
   - Fetches the latest receipts from the DHT for state verification.
   - Command: `sync`

### 2. Basic DHT Interaction
- Use the `libp2p` library for distributed hash table functionality.
- DHT Functions:
  - **Add Record**: Store transactions in the DHT.
  - **Retrieve Record**: Fetch recent transactions for validation.
- Support network interaction for peer-to-peer synchronization.

### 3. Data Storage
- Local storage of private keys, balances, and transaction history in a JSON file.
- Example structure for `wallet.json`:
  ```json
  {
    "public_key": "abc123",
    "private_key": "def456",
    "balance": 1000,
    "transactions": [
      {
        "to": "recipient_address",
        "amount": 50,
        "timestamp": "2024-12-28T10:00:00Z"
      }
    ]
  }
  ```

---

## Non-Functional Requirements

### 1. Programming Language
- **Rust**:
  - High performance.
  - Memory safety.
  - Libraries for cryptography and networking.

### 2. Cryptography
- Signature Algorithm: **Ed25519** (via `RustCrypto`).
- Hashing: **SHA-256** for generating unique transaction identifiers.

### 3. Stability
- Local data validation before broadcasting to DHT.
- Input validation to ensure sufficient funds for transactions.

### 4. CLI UX
- Simple command structure.
- User-friendly messages (confirmation of actions, error handling).

---

## Architecture

### 1. Modules
1. **Wallet Module**:
   - Key generation.
   - Balance management.
   - Saving and loading wallet data.

2. **Transaction Module**:
   - Receipt creation.
   - Transaction signing.
   - Balance and timestamp validation.

3. **DHT Module**:
   - `libp2p`-based peer discovery, storage, and receipt synchronization.

### 2. Workflow
1. User creates a wallet or loads an existing one.
2. Executes transactions broadcasted via DHT.
3. Client synchronizes network state to validate balances.

---

## Development Phases

### 1. Project Initialization
- Set up Rust project structure.
- Add core dependencies (`libp2p`, `serde`, `RustCrypto`).

### 2. Module Implementation
1. **Wallet**:
   - Key generation and wallet persistence.
2. **Transaction**:
   - Receipt creation and signing.
3. **DHT**:
   - Basic synchronization with `libp2p`.

### 3. CLI Interface
- Implement commands:
  - `create-wallet`, `show-wallet`, `send`, `sync`, `validate`.

### 4. Testing
- Local tests for cryptographic functions.
- Multi-client synchronization tests.

---

## Dependencies
- **libp2p**: For DHT functionality.
- **serde**: For data serialization/deserialization.
- **RustCrypto**: For cryptography.
- **clap**: For CLI command handling.

---

## Expected Outcome
A CLI client prototype that:
1. Allows wallet creation and basic operations.
2. Synchronizes with DHT for transaction storage and validation.
3. Demonstrates the Osanwe Wallet system in a real-world setting.

