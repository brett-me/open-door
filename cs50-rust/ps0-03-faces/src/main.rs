use std::io::stdin;

fn get_input() -> String {
    let mut user_input: String = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");
    user_input.trim().to_string()
}

fn replace_smiley(raw_input: String) -> String {
    raw_input.replace(":)", "😊").replace(":(", "🙁")
}

fn main() {
    let raw_input = get_input();
    println!("{}", replace_smiley(raw_input));
}
