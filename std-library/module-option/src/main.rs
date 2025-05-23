// Type Option represnts an optional value
// Every Option is either Some and contains a value, or None
// Options are commonly paired with pattern matching, accounting for None case

fn divide(numerator: f32, denominator: f32) -> Option<f32> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    // The return value of the function is an option
    let result = divide(2.0, 3.0);
    // Pattern match to retrieve the value
    match result {
        Some(x) => println!("Result: {x:.3}"),
        None => println!("Cannot divide by 0"),
    }
}
