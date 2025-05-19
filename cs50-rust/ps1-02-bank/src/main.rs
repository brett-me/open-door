use std::io::stdin;

fn main() {
    println!("Greeting:");
    let greeting: String = get_input();
    let cash: String = return_cash(&greeting); // pass a reference into return cash (function does not alter argument)
    println!("{cash}");
}

fn get_input() -> String {
    let mut greeting = String::new();
    stdin().read_line(&mut greeting).expect("String input");
    greeting.trim().to_lowercase() // no need to declare a new variable, apply methods directly to return value
}

fn return_cash(user_input: &str) -> String {
    match user_input {
        s if s.starts_with("hello") => "$0".into(), // nb: evoke variable s, so match arm is returnable
        s if s.starts_with("h") => "$20".into(),
        _ => "$100".into(), // _ serves as a catchall wildcard (must be the final match expression)
    }
}
