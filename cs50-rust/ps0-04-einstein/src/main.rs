use std::io::stdin;

fn main() {
    println!();
    println!("enter mass in kg (rounded to nearest kg): ");
    let mass = get_mass();
    println!();
    println!("energy = {} kilojoules", energy(mass));
}

fn get_mass() -> u64 {
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");
    user_input
        .trim()
        .parse::<u64>()
        .expect("Error, whole number not entered!")
}

fn energy(mass: u64) -> u64 {
    const SPEED_OF_LIGHT_MS: u64 = 299_792_458;
    (mass * SPEED_OF_LIGHT_MS) / 1000
}
