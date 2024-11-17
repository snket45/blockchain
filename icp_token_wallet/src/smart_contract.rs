pub struct SmartContract;

impl SmartContract {
    pub fn new() -> Self {
        Self {}
    }

    pub fn send(&self, recipient: &str, amount: u64) {
        // Simulated contract interaction
        println!("(SmartContract) Transferred {} tokens to {}.", amount, recipient);
    }

    pub fn fetch_balance(&self, _address: &str) -> u64 {
        // Simulated balance fetch
        1000
    }
}
