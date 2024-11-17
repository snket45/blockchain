use crate::smart_contract::SmartContract;

pub struct Wallet {
    balance: u64,
    contract: SmartContract,
}

impl Wallet {
    pub fn new() -> Self {
        Self {
            balance: 1000, // Initial balance for testing
            contract: SmartContract::new(),
        }
    }

    pub fn send_tokens(&mut self, recipient: &str, amount: u64) {
        if amount > self.balance {
            println!("Insufficient balance!");
            return;
        }

        self.contract.send(recipient, amount);
        self.balance -= amount;
        println!("Sent {} tokens to {}.", amount, recipient);
    }

    pub fn receive_tokens(&mut self, amount: u64) {
        self.balance += amount;
        println!("Received {} tokens.", amount);
    }

    pub fn display_balance(&self) {
        println!("Current balance: {} tokens", self.balance);
    }
}
