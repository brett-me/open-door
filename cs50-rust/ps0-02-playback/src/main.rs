#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

fn main() {
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");
    println!("{}", user_input.replace(' ', "..."));
}
