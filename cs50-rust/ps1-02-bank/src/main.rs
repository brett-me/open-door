use std::io::stdin;

fn main() {
    println!("Greeting: ");
    let greeting: String = get_input();
    let cash: String = return_cash(greeting);
    println!("{cash}");
}

fn get_input() -> String {
    let mut empty_line = String::new();
    stdin()
        .read_line(&mut empty_line)
        .expect("error reading line");
    let greeting: String = empty_line.trim().to_lowercase();
    greeting
}

fn return_cash(user_input: String) -> String {
    if user_input.starts_with("hello") {
        return "$0".into();
    } else if user_input.starts_with("h") {
        return "$20".into();
    } else {
        return "$100".into();
    };
}
