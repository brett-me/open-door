use std::io::stdin;

fn main() {
    println!("How much was the meal?");
    let dollars = get_user_input();
    let dollar_float = dollars_to_float(&dollars);

    println!("What percentage would you like to tip?");
    let percent = get_user_input();
    let percent_float = percent_to_float(&percent);

    let tip = dollar_float * (percent_float / 100.0);
    println!("Leave ${:.2}", tip);
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).expect("Read line error");
    user_input.trim().to_string()
}

fn dollars_to_float(user_input: &String) -> f32 {
    user_input
        .strip_prefix('$')
        .expect("Error, check format $##.##")
        .parse::<f32>()
        .expect("Error, check format $##.##")
}

fn percent_to_float(user_input: &String) -> f32 {
    user_input
        .strip_suffix('%')
        .expect("Error, check format #%")
        .parse::<f32>()
        .expect("Error, check format #%")
}
