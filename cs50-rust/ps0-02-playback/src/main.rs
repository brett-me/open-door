#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

fn main() {
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");

    let mut output = user_input;
    if output.ends_with('\n') {
        output.pop();
        if output.ends_with('\r') {
            output.pop();
        }
    }

    if output.contains(" ") {
        output = output.replace(" ", "...");
    }
    println!("{}", output);
}
