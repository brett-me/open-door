use std::io::{self, Write};

fn main() {
    println!("Expression:");
    io::stdout().flush().unwrap();
    let expression = get_expression();
    println!("{:.1}", calculate(&expression));
}

fn get_expression() -> String {
    let mut expression = String::new();
    io::stdin()
        .read_line(&mut expression)
        .expect("int (+, -, *, /) int");
    expression.trim().to_lowercase()
}

fn calculate(expression: &str) -> f32 {
    let elements: Vec<&str> = expression.split_whitespace().collect();
    if elements.len() != 3 {
        return 0.0;
    }
    // use match to parse element, this avoids panic!
    let x: f32 = match elements[0].parse() {
        Ok(n) => n,
        Err(_) => return 0.0,
    };
    let y: f32 = match elements[2].parse() {
        Ok(n) => n,
        Err(_) => return 0.0,
    };
    let operator = elements[1];
    match operator {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => 0.0,
    }
}
