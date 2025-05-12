#![warn(clippy::all, clippy::pedantic)]

// gathering input: step 2
use std::io::stdin;

fn main() {
    println!("Hello, what's your name?");
    // gathering input: step 1
    let mut your_name = String::new();
    // gathering input: step 3
    stdin() // returns an object granting access to Standard Input
        .read_line(&mut your_name) // stdin method, recieves keyboard input until <CR>
        .expect("Failed to read line"); // unwrap result object, terminate programme if error
    println!("Hello, {}", your_name);
}
// ------------------------------------------------------------------------------------------------
// Gathering user input
// ------------------------------------------------------------------------------------------------
// 1. create a mutable variable and bind it to an empty string
// 2. enable use of the Standard Input functions
// 3. access stdin and function chain .read_line and expect
//      - .read_line takes a mutable reference to the variable bound to an empty string in step 1
//      - .expect takes a message that will output if there is an error
