#[cfg(test)]
mod tests {
    use super::super::{smart_contract::SmartContract, wallet::Wallet};

    #[test]
    fn test_send_tokens() {
        let mut wallet = Wallet::new();
        wallet.send_tokens("recipient1", 200);
        assert_eq!(wallet.balance, 800);
    }

    #[test]
    fn test_receive_tokens() {
        let mut wallet = Wallet::new();
        wallet.receive_tokens(300);
        assert_eq!(wallet.balance, 1300);
    }

    #[test]
    fn test_insufficient_balance() {
        let mut wallet = Wallet::new();
        wallet.send_tokens("recipient1", 1200); // Exceeds balance
        assert_eq!(wallet.balance, 1000);
    }
}
