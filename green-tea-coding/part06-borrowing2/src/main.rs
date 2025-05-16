fn main() {
    let input: String = String::from("input");
    transfer_ownership(input); // input no longer valid
    let input: String = String::from("input"); // variable input declared again (new binding)
    immutable_reference(&input);
    println!("Original owner, {}", input); // variable input still valid
    let mut input2: String = String::from("input2");
    mutable_reference(&mut input2);
    println!("Original owner, {}", input2); // original structure valid, but has changed
}

// takes ownership of data structure, orgininal binding no longer valid
fn transfer_ownership(mut input: String) {
    input.push_str("/output");
    println!("transfer ownership of {}", input);
}

// readable reference to the data structure, original binding still valid and unchanged
fn immutable_reference(input: &String) {
    println!("immutable reference of {}", input);
}

// writable reference to the data structire, original binding still valid BUT changed
fn mutable_reference(input: &mut String) {
    input.push_str("/output2");
    println!("mutable reference of {}", input);
}
