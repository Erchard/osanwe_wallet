pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
}

impl Transaction {
    pub fn new(sender: &str, recipient: &str, amount: u64) -> Self {
        Transaction {
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount,
        }
    }

    pub fn sign(&self, _private_key: &str) {
        // Реалізація буде додана пізніше
    }

    pub fn verify_signature(&self) -> bool {
        // TODO: Implement signature verification
        true
    }
}
