use std::io::{self, Write};

fn main() {
    give_change();
}

fn give_change() {
    let mut amount_due: i32 = 50;
    while amount_due > 0 {
        println!("Amount Due: {}", amount_due);
        print!("Insert Coin: ");
        io::stdout().flush().unwrap();

        let mut coin = String::new();
        io::stdin()
            .read_line(&mut coin)
            .expect("Error reading line");

        let coin: &str = coin.as_str().trim();
        if coin == "25" || coin == "10" || coin == "5" {
            let coin: i32 = coin.parse().expect("number");
            amount_due -= coin
        }
    }
    let change: i32 = amount_due.abs();
    println!("Change Owed: {}", change);
}
