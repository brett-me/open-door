// ================================================================================================
//  CH04-02 REFERENCES AND BORROWING
// ================================================================================================
// functions drop/transfer ownership of arguments passed into them
// to keep the original variable valid (usable post function), pass a reference into the function
// references are denoted with &
// a reference acts as a pointer to the pointer
// references are dropped when they go out of scope
// creating a reference is called BORROWING
// references are immutable by default

fn main() {
    println!();
    let s1 = String::from("A reference to this variable was passed through a function");
    let len = calculate_length(&s1); // a reference to s1 is passed as an argument
    println!("'{s1}' has {len} characters"); // s1 is still valid and is used here
    println!();
    let mut intro = String::from("Hi, my name is... "); // here, variable declared mutable
    mutate_reference(&mut intro); // mutable reference passed as argument
    println!("{}", intro); // intro is valid here and can be used
    println!();
    error_two_mut_ref_to_same_value();
    println!();
    error_mutable_and_immutable_ref_same_value();
    println!();
    mutiple_immutable_references_to_same_value();
}

// this function has a reference parameter
fn calculate_length(s: &String) -> usize {
    s.len()
}

// ================================================================================================
//  mutable references
// ================================================================================================
// parameter explicitly denoted as a mutable reference
// nb: original variable must also be declared as mutable
// when called (see above) argument must also be denoted as a mutable reference

fn mutate_reference(reference_to_string: &mut String) {
    reference_to_string.push_str("Leroy")
}

// NB!!! There cannot be more than one mutable reference to a value at the same time
fn error_two_mut_ref_to_same_value() {
    let mut s = String::from("there can only be one mutable reference to a value at the same time");
    let mutable_reference1 = &mut s; // first MUTABLE reference to s
    // ERROR: let reference2 = &mut s; // second MUTABLE reference to s
    // ERROR: println!("{}, {}", reference1, reference2);
    println!("{}", mutable_reference1);
}

// BUT, not immutable and mutable reference to the same value!!!
fn error_mutable_and_immutable_ref_same_value() {
    let mut s = String::from(
        "there cannot be a mutable and immutable reference to the same value at the same time",
    );
    let mutable_reference1 = &mut s;
    // ERROR: let immutable_reference2 = &s;
    println!("{}", mutable_reference1);
}

// however, there can be multiple immutable references to a value simulataneously
fn mutiple_immutable_references_to_same_value() {
    let s = String::from("Multiple references to this");
    let immutable_reference1 = &s;
    let immutable_reference2 = &s;
    println!(
        "Original: {}, immutable reference 1: {}, immutable reference 2: {}",
        s, immutable_reference1, immutable_reference2
    );
}
// 'data races'
// ================================================================================================
// more than one pointer can access data at the same time
// when at least one pointer is writing data
// there is no mechanism used to synchronise access to data

// ================================================================================================
//  dangling references
// ================================================================================================
// in Rust, the original value will never go out of scope before the reference
// also, it's not possible to reference a value that does not exist

// ================================================================================================
//  references rules recap
// ================================================================================================
// 1. at any given time, Rust allows either one mutable reference or multiple references
// 2. references must always be valid
// 1 prevents data races, 2 prevents dangling references
