// ================================================================================================
// CH 06-02 MATCH CONTROL FLOW CONSTRUCT
// ================================================================================================
// compares a value against a series of patterns and then executes code based on match
// patterns can be made up of literal values, variable names, wildcards (and other things)

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        } // more than one expression can be returned
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        // match arm... pattern => expression
    }
}

fn main() {
    let penny = Coin::Penny;
    println!("{}", value_in_cents(penny));
}
