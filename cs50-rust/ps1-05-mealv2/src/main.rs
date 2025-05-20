use regex::Regex;
use std::{
    f32,
    io::{self, Write},
};

fn main() {
    match get_valid_input() {
        Some(input) => response(&input),
        None => println!("Exiting on request..."),
    }
}

fn get_valid_input() -> Option<Vec<u8>> {
    // time format: hh:mm
    let time_format = Regex::new(r"^\d{2}:\d{2}$").unwrap();
    loop {
        // request and store user input
        print!("What time is it (format hh:mm or 'q' to quit): ");
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        let user_input = user_input.trim();
        // quit option
        if user_input == "q" {
            return None;
        }
        // check user input against time format
        if time_format.is_match(user_input) {
            // convert input String to Vec u8
            let elements: Vec<u8> = user_input
                .split(':')
                .filter_map(|s| s.parse::<u8>().ok())
                .collect();
            // check valid time entered
            if elements.len() == 2 && elements[0] < 24 && elements[1] < 60 {
                // break loop and return elements if valid
                return Some(elements);
            }
        }
        // error message input invalid, loop starts again
        println!("Invalid time format.")
    }
}

fn response(user_input: &Vec<u8>) {
    // convert to float because 8am, 1pm and 7pm are inclusive meal times
    let hours: f32 = user_input[0] as f32;
    let minutes: f32 = user_input[1] as f32;
    let time_as_float: f32 = hours + (minutes / 60.0);
    match time_as_float {
        7.0..=8.0 => println!("breakfast time"),
        12.0..=13.0 => println!("lunch time"),
        18.0..=19.0 => println!("dinner time"),
        _ => println!("here's a snack"),
    }
}
