pub struct Wallet {
    pub public_key: String,
    pub private_key: String,
    pub balance: i64,
}

impl Wallet {
    pub fn new(public_key: String, private_key: String) -> Self {
        Wallet {
            public_key,
            private_key,
            balance: 0,
        }
    }

    pub fn load_from_file(_path: &str) {
        // Реалізація буде додана пізніше
    }

    pub fn save_to_file(_path: &str) {
        // Реалізація буде додана пізніше
    }

    pub fn update_balance(&mut self, amount: i64) {
        self.balance += amount;
    }
}
