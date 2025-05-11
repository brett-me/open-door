// ================================================================================================
//   Problem Set 0: Indoor Voice
// ================================================================================================
// implement a program that prompts the user for input and then
// outputs that same input in lowercase.

use std::io;

fn main() {
    println!();
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");
    let quiet = user_input.trim().to_lowercase();
    println!("{}", quiet);
}
