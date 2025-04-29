// ================================================================================================
//   ch02: COMPOUND DATA TYPES
// ================================================================================================
// arrays, tuples, slices, Strings (and string references --&str)

fn main() {
    array_type(); // fixed size, homogeneous types
    tuple_type(); // fixed size, heterogeneous types
    slice_type(); // dynamic size, sequence contiguous elements
    string_type(); // dynamic, growable, allocated on heap
    string_reference_type(); // static, reference (slice), allocated on stack
}

// ================================================================================================
//  arrays
// ================================================================================================
// fixed size collection of homogeneous types

fn array_type() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // array annotation [data type; array scope]
    println!("array of numbers: {:?}", numbers); // {:?} debugable format
    let fruit: [&str; 3] = ["apple", "banana", "orange"];
    println!("index into strings: {}", fruit[1]); // slice
    let shorthand = [0; 10]; // [element; number of occurances]
    println!("{:?}", shorthand);
}

// ================================================================================================
//  tuples
// ================================================================================================
// fixed size collection of heterogeneous types

fn tuple_type() {
    // tuple with int, str, bool, and array
    let t: (i32, String, bool, [i32; 3]) = (100, "string".to_string(), true, [10, 20, 30]);
    println!("tuple of different types: {:?}", t);
}

// ================================================================================================
//  slices
// ================================================================================================
// a contiguous sequence of elements
// dynamically sized

fn slice_type() {
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("slice of numbers: {:?}", number_slices);
    let animal_slices: &[&str] = &["cat", "dog", "fish"];
    println!("slice of string references: {:?}", animal_slices);
    let name_slices: &[&String] = &[&"leroy".to_string(), &"jackson".to_string()];
    println!("slice of Strings: {:?}", name_slices)
}

// ================================================================================================
//  Strings (and string references)
// ================================================================================================
// Strings are mutable --growable (increase or decrease in size)
// Strings are 'owned'
// allocated on the heap
// string refereces (&str)

fn string_type() {
    let mut name: String = String::from("john");
    println!("{name}");
    name.push_str(" cena"); // push_str appends to String
    println!("{name}"); // updated String ('brett smith')
}

// ================================================================================================
//  string references (&str)
// ================================================================================================
// stored on stack as a reference
// immutable
// fixed size

fn string_reference_type() {
    let hello: String = String::from("Hello, World!");
    let string_reference: &str = &hello[0..=4];
    println!("{string_reference}");
}
