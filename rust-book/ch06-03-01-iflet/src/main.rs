// if let, short-hand notation when we don't want to add functionality to None
// reduces boilerplate code: _ => (),

#![allow(dead_code)]

fn main() {
    // example 1: match boilerplate
    println!();
    println!("example 1-1: ");
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // example 1: if let
    println!();
    println!("example 1-2: ");
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // example 2: match boilerplate
    println!();
    println!("example 2-1: ");
    let coin = Coin::Quarter(State::Alaska);
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    // example 2 if let else
    println!();
    println!("example 2-2: ");
    let coin = Coin::Quarter(State::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    // example 3: if let ... let else in a function
    println!();
    println!("example 3-1:");
    let coin = Coin::Quarter(State::Alaska);
    let description = describe_state_quarter(coin);
    println!("{:?}", description);
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(State),
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
}

impl State {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            State::Alabama => year >= 1819,
            State::Alaska => year >= 1959,
        }
    }
}
// function uses if let and let...else
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
