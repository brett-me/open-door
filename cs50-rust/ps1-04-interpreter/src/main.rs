use std::io::stdin;

fn main() {
    println!("Expression:");
    let expression = get_expression();
    println!("{:.1}", calculate(&expression));
}

fn get_expression() -> String {
    let mut expression = String::new();
    stdin()
        .read_line(&mut expression)
        .expect("int (+, -, *, /) int");
    expression.trim().to_lowercase()
}

fn calculate(expression: &str) -> f32 {
    let elements: Vec<&str> = expression.split(' ').collect();
    if elements.len() != 3 {
        return 0.0;
    } else {
        let x: f32 = elements[0].parse::<f32>().expect("number");
        let operator = elements[1];
        let y: f32 = elements[2].parse::<f32>().expect("number");
        match operator {
            "+" => x + y,
            "-" => x - y,
            "*" => x * y,
            "/" => x / y,
            _ => 0.0,
        }
    }
}
