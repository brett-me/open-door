// ================================================================================================
//  CH04-01 OWNERSHIP
// ================================================================================================
// ownership is a set of rules that govern how a Rust programme manages memory
// if any of the rules are violated, the programme won't compile

fn main() {
    println!();
    scope_example();
    string_example();
    println!();
    move_example(); // automatic transfer of ownership
    shadow_drop(); // automatic drop of ownership
    mutate_drop(); // automatic drop of ownership
    println!();
    copy_example(); // automatic, applied to fixed-size variables on stack
    clone_example(); // explicit, applied to dynamic variables on allocated on heap
    println!();
    let s: String = String::from("This variable (argument) moved ownership and was dropped");
    takes_ownership(s); // after this function, s is no longer valid (moved and dropped)
    //  println!("{s}"); this line generates a compile error
    let n: u8 = 100; // fixed-size variable type
    makes_copy(n); // a copy of n (fixed-size) is used in the function, n remains valid
    println!("{n}"); // this is possible
    println!();
    let takes = gives_ownership();
    println!("{takes}");
    let s2 = String::from("This is s2");
    let s3 = takes_and_gives_back(s2);
    println!("{s3}");
    println!();
    let this_variable_passes_through = String::from("this variable passed through the function");
    let (takes_passed_value, len_of_passed_value) = calculate_length(this_variable_passes_through);
    println!("{takes_passed_value}, and has a length of {len_of_passed_value} characters");
}

// ================================================================================================
//  stack and heap
// ================================================================================================
// the stack and the heap are parts of memory available to your programme at runtime
// stack more effiencent, reduces jumping around memory and associated overhead

// stack
// ================================================================================================
// all data stored on the stack must have a known, fixed size
// the stack stores values in the order it receives them, and removes values in opposite order
// referred to as last in, first out
// adding data is called pushing onto the stack
// removing data is called popping off

// heap
// ================================================================================================
// the heap is less organised
// request amount of space, memory allocator finds location, returns a pointer (location address)
// pointer can be stored on stack
// allocating on the heap is slower: search for location, remember location

// ================================================================================================
//  ownership rules
// ================================================================================================
// 1. each value in Rust has an owner
// 2. there can only be one owner at a time
// 3. when the owner goes out of scope, the value will be dropped

// ================================================================================================
// variable scope (and string literal / reference)
// ================================================================================================
// string literal is stored in static read-only memory and is immutable
// here, string_reference references a string literal in static read only memory.
// both the string literal and reference are hardcoded into memory at compile time

fn scope_example() {
    // string_literal is not valid here, it hasn't been declared yet
    let string_literal: &'static str = "hello string literal (&static' str)";
    // string_literal valid from this point
    println!("{string_literal}");
    let string_reference: &str = "hello string reference (&str)";
    println!("{string_reference}");
} // the scope is over, string_literal is no longer valid

// ================================================================================================
// String type
// ================================================================================================
// complex data type, dynamic, allocated on heap
// here, example owns the string "hello" and it is mutable
// memory requested from allocator at runtime and automatically returned when out of scope
fn string_example() {
    let mut string_type: String = String::from("hello String type");
    println!("{string_type}");
    string_type.push_str(" (String::from(''))");
    println!("{string_type}");
}

// ================================================================================================
// move (automatic ownership TRANSFER)
// ================================================================================================

fn move_example() {
    let s1: String = String::from("This value moved from s1 to s2, making s1 invalid");
    let s2 = s1;
    // println!("{s1}"); THIS RESULTS IN A COMPILE ERROR, S1 no longer valid
    println!("{s2}");
}

// ================================================================================================
// drop
// ================================================================================================
// Rust automatically drops ownership when a variable is out of scope

fn shadow_drop() {
    let x: u8 = 100;
    println!("variable x before we shadow and drop it, {x}");
    // if we didn't use x, advisable to prefix it with an underscore (_x)
    let x: u8 = 255; // value 100 is no longer valid, it was dropped
    println!("here is an example of how values are dropped when they are shadowed, x = {x}");
}

fn mutate_drop() {
    let mut s: String = String::from("hello");
    println!("variable s before we mutate and drop it, {s}");
    s = String::from("ahoy"); // value "hello" is no longer valid, it was dropped
    println!("{s}, world");
}

// ================================================================================================
// copy (fixed-size data types, stack) vs clone (dynamic data types, heap)
// ================================================================================================
// an example illustrating AUTOMATIC copy of fixed-size variables
fn copy_example() {
    let x: u8 = 5;
    let y = x; // value of x copied to y
    println!("x = {x}"); // x retains ownership of value 5 and is still valid
    println!("y = {y} ... I'm a copy of x!"); // y is a copy of x and is valid
}

// an example illustrating EXPLICIT cloning of dynamic variables;
// heavy performance cost, cloning allocations on heap!!!
fn clone_example() {
    let s1: String = String::from("hello");
    let s2 = s1.clone(); // clone needs to be exlplictly invoked
    println!("s1 = {s1}, s2 = {s2}");
}

// ================================================================================================
// ownership and functions
// ================================================================================================
// passing a variable to a function will move (and drop) dynamic variables or copy static

// dynamic parameter, ownership of variable (argument) moved and dropped
fn takes_ownership(s: String) {
    println!("{s}");
}

// static parameter: copy of variable (argument) made
fn makes_copy(n: u8) {
    println!("{n}");
}

// ================================================================================================
// return values and scope
// ================================================================================================
// returning values can transfer ownership
// non-returning functions drop ownership

fn gives_ownership() -> String {
    let s: String = String::from("ownership received from a function");
    s
}

fn takes_and_gives_back(s2: String) -> String {
    s2
}

fn calculate_length(a_string: String) -> (String, usize) {
    let string_length: usize = a_string.len();
    (a_string, string_length)
}
