use std::io::{self, Write};

fn main() {
    print!("Input: ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("input should be text");
    let user_input: &str = user_input.trim();
    let mut output = String::new();
    for c in user_input.chars() {
        if matches!(c, 'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U') {
            continue;
        } else {
            output.push(c);
        }
    }
    println!("Output: {}", output);
}
