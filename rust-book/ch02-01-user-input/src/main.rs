// ================================================================================================
//   CH02-01: USER INPUT
// ================================================================================================
// chapter 2 in The Book walks through the code for a guessing game
// it gathers user input, which is the sole focus of this (abbridged) version
// content from gemini 2.5 flash

use std::io;

fn main() {
    println!("Please enter a number:");

    // declare a mutable String and initialise it as empty
    let mut user_input = String::new();

    // get standard input stream
    io::stdin()
        // read a line of text from the input stream and append it to the user string
        .read_line(&mut user_input)
        // basic error handling
        .expect("Failed to read line");

    let number: i32 = user_input
        .trim()
        .parse()
        .expect("Please enter a valid number");
    // trim: removes leading and trailing white spaces, including the newline character
    // parse: attempts to convert value type and return value
    // expect: basic error handling if parse fails
    println!("You entered the number: {}", number);
}
