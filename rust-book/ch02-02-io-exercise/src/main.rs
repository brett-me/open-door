// ================================================================================================
//   CH02-02: IO EXERCISE
// ================================================================================================
// practice implementing simple IO code

use std::io;

fn main() {
    println!(); // design preference
    println!("What's your name: ");
    // initialise empty, mutable String and bind to variable 'user_name'
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let user_name = user_input.trim();
    println!("Hi, {}", user_name);
}
