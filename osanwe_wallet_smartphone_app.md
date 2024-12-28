
# Smartphone Application for Osanwe Wallet

## Overview
The Osanwe Wallet smartphone application provides a user-friendly interface for interacting with the decentralized microtransaction system. The app integrates the Rust-based backend library to handle cryptographic operations and DHT communication while offering an intuitive graphical user interface for seamless wallet management.

---

## Technology Stack

### Backend (Rust Library)
- **Programming Language**: Rust
- **Core Functionality**:
  - Wallet management (key generation, balance updates, transaction creation).
  - DHT communication using `libp2p`.
  - Cryptographic operations using `ed25519-dalek`.

### Mobile Framework
- **Framework**: Flutter
  - Chosen for its cross-platform capabilities, allowing deployment on both Android and iOS.
  - Provides a rich set of UI components for creating responsive designs.
- **Integration**: Rust library integrated into Flutter via `flutter_rust_bridge`.

### UI Design Tools
- **UI Framework**: Material Design (for Android) and Cupertino (for iOS).
- **Graphic Components**: Custom widgets for transaction history, QR code scanning, and wallet operations.

---

## Integration with Rust Library

### Rust Library Details
- The Rust library provides core functionalities:
  - Generating and managing Ed25519 key pairs.
  - Signing and verifying transactions.
  - Synchronizing with the DHT network.

### Communication between Flutter and Rust
- **Tool**: `flutter_rust_bridge`
  - Bridges the Dart and Rust codebases.
  - Allows seamless calling of Rust functions from Flutter.

### Steps to Integrate
1. Build the Rust library as a dynamic shared library (`.so` for Android, `.dylib` for iOS).
2. Add the shared library to the Flutter project under platform-specific folders.
3. Use `flutter_rust_bridge` to define the interface between Dart and Rust functions.

Example of Rust function:
```rust
#[flutter_rust_bridge]
pub fn generate_wallet() -> Wallet {
    // Logic for key generation and wallet initialization
}
```

Example of Dart call:
```dart
final wallet = await api.generateWallet();
print("Public Key: ${wallet.publicKey}");
```

---

## User Interface

### Key Screens
1. **Home Screen**:
   - Displays wallet balance, recent transactions, and quick action buttons (e.g., Send, Receive).
2. **Send Screen**:
   - Allows the user to enter recipient address and amount.
   - Includes a QR code scanner for recipient address.
3. **Receive Screen**:
   - Shows the user's public key as a QR code.
   - Allows copying the public key to the clipboard.
4. **Transaction History**:
   - Displays a list of past transactions with details (amount, recipient, timestamp).
5. **Settings Screen**:
   - Options for wallet backup, key rotation, and syncing with the DHT network.

### UI Components
- **Custom Widgets**:
  - Wallet balance card with animations.
  - Transaction list with expandable details.
  - QR code scanner and generator using `flutter_qr_bar_scanner`.

### Security Features
- **Biometric Authentication**:
  - Fingerprint or Face ID authentication for app access.
- **Encrypted Storage**:
  - Store private keys securely using platform-specific secure storage APIs.

---

## User Guide

### Setting Up the Wallet
1. **Install the App**:
   - Download from Google Play Store or Apple App Store.
2. **Create a Wallet**:
   - Upon first launch, the app generates a new wallet and displays the public key.
   - Backup your private key securely (not shown directly but saved in encrypted storage).
3. **View Wallet Details**:
   - Navigate to the Home Screen to view balance and recent transactions.

### Sending Funds
1. Go to the Send Screen.
2. Enter the recipient's public key or scan their QR code.
3. Enter the amount to send.
4. Confirm the transaction. The app signs the transaction and broadcasts it to the DHT network.
5. View the transaction status in the Transaction History screen.

### Receiving Funds
1. Go to the Receive Screen.
2. Share your public key as a QR code or copy it to share manually.
3. Wait for the sender to complete the transaction. Synchronize the app to fetch the updated balance.

### Synchronizing with the Network
1. Open the Settings Screen.
2. Tap "Synchronize" to fetch the latest state from the DHT.

### Backing Up and Restoring Wallets
1. **Backup**:
   - Export the encrypted wallet file from the Settings Screen.
2. **Restore**:
   - Import the encrypted wallet file during the app setup.

### Security Notes
- Never share your private key.
- Enable biometric authentication for added security.

---

## Deployment Plan

1. **Prepare Backend Library**:
   - Compile the Rust library for Android and iOS platforms.
   - Test the library functions using `flutter_rust_bridge` bindings.

2. **Develop Flutter App**:
   - Create UI screens using Flutter widgets.
   - Integrate the Rust backend for wallet operations.

3. **Testing**:
   - Unit test Rust functions and Dart code.
   - Perform integration tests between Flutter and Rust.

4. **Publishing**:
   - Submit the app to Google Play Store and Apple App Store.

---

## Potential Improvements
1. **Dark Mode**:
   - Add theme toggling for better user experience.
2. **Advanced Analytics**:
   - Display spending patterns and trends in the Home Screen.
3. **Offline Transactions**:
   - Allow transaction signing offline and broadcast when online.

---

This document outlines the detailed structure, integration process, and user instructions for the Osanwe Wallet smartphone application. It ensures a seamless experience for users while leveraging the power of decentralized technologies.
