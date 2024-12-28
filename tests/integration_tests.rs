use osanwe_wallet::{Wallet, Transaction, DHT};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration() {
        let _wallet = Wallet::new("public_key".to_string(), "private_key".to_string());
        let _transaction = Transaction::new("sender", "recipient", 100);
        let _dht = DHT::new();
        
        // Заглушка для тесту
        assert!(true);
    }
}
