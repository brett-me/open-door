// BORROWING GUIDELINES:
// 1. we can have any number of read only (immutable) references to a value
// 2. only 1 active mutable reference at any given time
// 3. references must always be valid

fn main() {
    println!();
    // any number of immutable references to the same value at the same time
    let message1: String = String::from("Hello"); // message1 allocated on heap
    let message2: &String = &message1; // message2 is a read-only reference of message 1
    println!("message1: {} | message2: {}", message1, message2); // message1 still valid
    let message3: &String = &message1; // we can have any number of read only references
    println!(
        "message! {} | message2: {} | message3: {}",
        message1, message2, message3
    );
    println!();
    // only 1 mutable reference to the same value at the same time
    let mut number: String = String::from("1"); // remember to add mut, even to dynamic types, default is immutable
    println!("{number}");
    number.push_str(", 2"); // we can push to the string because it is mutable
    println!("{number}");
    let reference: &mut String = &mut number; // creating a mutable reference to variable 'number'
    reference.push_str(", 3"); // we can push to the reference because it is mutable
    println!("{reference}");
    println!("{reference}");
    println!("{number}"); // changing reference changed number
    // number is active, making reference invalid
}
