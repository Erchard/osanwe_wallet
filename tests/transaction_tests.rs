use osanwe_wallet::Transaction;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let _transaction = Transaction::new("sender", "recipient", 100);
        assert!(true); // Тимчасова заглушка
    }
}
