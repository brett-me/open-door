#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

fn get_name() -> String {
    let mut user_name = String::new();
    stdin()
        .read_line(&mut user_name)
        .expect("Error reading line");
    user_name.trim().to_lowercase()
}

fn main() {
    println!("What's your name: ");
    let name = get_name();
    println!("Hello, {}", name);

    let visiter_list: [&str; 3] = ["leroy", "caron", "roan"];
    let mut allow_them_in = false;
    for visiter in &visiter_list {
        if visiter == &name {
            allow_them_in = true;
        }
    }
    if allow_them_in {
        println!("welcome to the treehouse, {}", name);
    } else {
        println!("Access denied, smelly fart!");
    }
}
