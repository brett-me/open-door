// ================================================================================================
//   ch04: OWNERSHIP
// ================================================================================================
// 1. each value has an owner
// 2. there can only be one owner at a time
// 3. when an owner goes out of scope, the value is dropped

fn main() {
    let s1: String = String::from("rust");
    // a reference to s1 is passed as an argument to function calculate_length
    // s1 preserves ownership of "rust"
    println!("S1: The length of word '{s1}' is {}", calculate_length(&s1));
    let s2: String = s1; // ownership moved from s1 to s2 (shadowing)
    // println!("{s1}"); this produces an error because s1 is out of scope
    println!("S2: The length of word '{s2}' is {}", calculate_length(&s2));
    let _my_details = {
        let name = "brett";
        let age = 40;
        let location = "cape town";
        println!("{name}, {age}, {location}")
    };
    // println!("{name}"); this produces an error because ownership of name is dropped post scope
}

fn calculate_length(s: &String) -> usize {
    // parameter is a reference (& denotes a reference, and is placed before declaring type)
    s.len()
}
