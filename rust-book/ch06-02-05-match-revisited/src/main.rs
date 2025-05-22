#![allow(dead_code)]

fn main() {
    let penny = Coin::Penny;
    println!("{}", value_in_cents(penny));
    let wife = MyFamily::MyLove(Name::Caron(String::from("simply the best")));
    message(wife);
    let pet = MyFamily::Cat(Name::Leroy(String::from("adorable")));
    message(pet)
}

// define enum Coin with 4 variants (Penny, Nickel, Dime, Quarter)
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// define function that returns a u8 type based on an enum match
// note: all enum variants exhausted
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            // match arm code can be several lines long
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum MyFamily {
    // enum variants can hold a different enum variant as a value
    MyLove(Name), // Name is an enum with it's own variants
    Cat(Name),
}

// derive debug added so enum variant can be printed
#[derive(Debug)]
enum Name {
    Caron(String),
    Leroy(String),
}

fn message(family: MyFamily) {
    match family {
        MyFamily::MyLove(name) => println!("I love you {name:?}"),
        MyFamily::Cat(name) => println!("You're a poop, {name:?}"),
    }
}
