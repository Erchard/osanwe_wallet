use osanwe_wallet::Wallet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let _wallet = Wallet::new("public_key".to_string(), "private_key".to_string());
        assert!(true); // Тимчасова заглушка
    }
}
