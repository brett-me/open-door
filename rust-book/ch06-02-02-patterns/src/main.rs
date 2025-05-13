// ================================================================================================
// CH 06-02-02 PATTERNS THAT BIND TO VALUES
// ================================================================================================
// match arms can bind to the parts of the values that match the pattern

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickle(UsState),
}

fn value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle(state) => {
            println!("State nickle from {state:?}!");
            5
        }
    }
}

fn main() {
    value(Coin::Nickle(UsState::Alaska));
}
