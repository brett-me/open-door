use std::io::{self, Write};

fn main() {
    let input = get_input();
    println!("{}", is_valid(&input));
}

fn get_input() -> String {
    print!("Plate: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Text input as String");
    input = input.trim().to_uppercase().to_string();

    input
}

fn is_valid(input: &str) -> bool {
    if input.len() < 2 || input.len() > 6 {
        false
    } else {
        true
    }
}
