use std::io::stdin;

fn main() {
    println!();
    // stack: staticly sized data, details known at compile time
    let a: i32 = 100; // here, a is declared as an i32 typed- size known at compile time
    let b = a; // because variable a is on the stack, it's value is COPIED to b
    println!("An example of storing static data on the stack, copying and reusing data");
    println!("println!(a: {{}}, b: {{}}, a, b)");
    println!("a: {}, b: {} ... value COPIED and a is still valid", a, b); // a is still valid and can be passed into the println! macros
    println!();

    {
        // when data is out of scope, it gets pushed off the stack
        let c: i32 = 5000; // here, variable c is statically sized, known at compile time and pushed onto the stack
        println!("Here, c is on the stack, but defined within it's own scope");
        println!("This is C, being called within it's own scope: {}", c);
        println!("C will be pushed off the stack outside of this scope and will become invalid");
        println!();
    }

    // c is now out of scope. It's no  longer on the stack and cannot be used again
    // println!("{c}"); this line results in a compile error and the programme won't run
    println!("variable C is out of scope here, the code below would produce an error: ");
    println!("println!({{c}})");
    println!();

    // heap: dynamically sized data, details change during run time
    println!("Exploring dynamic data on the heap, enter a phrase:");
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).expect("Error"); // here, user_input is dynamic and determined during runtime
    let move_input = user_input; // because variable user_input is allocated on heap, it's value is MOVED to move_input
    println!(
        "user_input no longer valid, MOVED to move_input: {}",
        move_input
    ); // user_input no longer valid and cannot be passed into println! macros without a compiler error
    println!("This code is not valid: ");
    println!("println!({{user_input}} and {{move_input}})");
}
