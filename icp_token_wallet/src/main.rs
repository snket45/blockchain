mod wallet;
mod smart_contract;
mod tests;

use wallet::Wallet;
use std::io;

fn main() {
    let mut wallet = Wallet::new();

    println!("ICP Token Wallet");
    loop {
        println!("\n1. Display Balance\n2. Send Tokens\n3. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => wallet.display_balance(),
            "2" => {
                let mut recipient = String::new();
                let mut amount = String::new();

                println!("Enter recipient address:");
                io::stdin().read_line(&mut recipient).unwrap();

                println!("Enter amount:");
                io::stdin().read_line(&mut amount).unwrap();

                let amount: u64 = amount.trim().parse().unwrap();
                wallet.send_tokens(&recipient.trim(), amount);
            }
            "3" => break,
            _ => println!("Invalid choice!"),
        }
    }
}
