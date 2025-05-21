use std::io::{self, Write};

fn main() {
    println!("{}", convert_snakecase(get_input()));
}

fn get_input() -> String {
    print!("Text: ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let user_input = user_input.trim().to_string();
    user_input
}

fn convert_snakecase(user_input: String) -> String {
    let mut snake_case = String::new();
    for (i, c) in user_input.chars().enumerate() {
        if i != 0 && c.is_uppercase() {
            snake_case.push('_');
        }
        snake_case.push(c.to_ascii_lowercase());
    }
    snake_case
}
