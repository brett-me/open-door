use std::io::stdin;

fn main() {
    println!();
    let great_answer_life: [&str; 4] = ["42", "forty-two", "forty two", "my love caron"];

    println!("What is the Answer to the Great Question of Life, the Universe, and Everything?");
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");

    let input_lowered = user_input.trim().to_lowercase();

    if great_answer_life.contains(&input_lowered.as_str()) {
        println!("Yes")
    } else {
        println!("No")
    }
}
