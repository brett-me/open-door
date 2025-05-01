use std::u16;

// ================================================================================================
//   CH03-02: DATA TYPES
// ================================================================================================
//  every value in Rust is a certain data type

fn main() {
    println!();
    integers_table();
    rust_char();
    tuple_example();
    destructure_tuple_target();
    direct_access_tuple_element();
}

// ================================================================================================
//  scalar types
// ================================================================================================
// represent a single value
// 4 primary types: integers, floating-point numbers, Booleans, and characters
//
// ================================================================================================
// 1. integers
// ================================================================================================
// signed (i) includes both negative and positive numbers
// from -2**(n-1) to 2**(n-1) - 1

// unsigned (u), only positive numbers
// from 0 to 2**n - 1 (note that u can store a larger positive number using same memory capacity)

// arch (i/usize) depends on architecture or pc running programme (ie: typically 64 or 32 bits)

// non-annotated integer declarations default to i32
fn integers_table() {
    let i8_max: i8 = 127;
    let u8_max: u8 = 255;
    let i16_max: i16 = 32767;
    let u16_max: u16 = 65535;
    let i32_max: i32 = 2147483647;
    let u32_max: u64 = 4294965295;
    let i64_max: i64 = 9223372036854775807;
    let u64_max: u64 = 18446744073709551615;
    println!("{:>3} max: {:>20}", "i8", i8_max);
    println!("{:>3} max: {:>20}", "u8", u8_max);
    println!("i16 max: {:>20}", i16_max);
    println!("u16 max: {:>20}", u16_max);
    println!("i32 max: {:>20}", i32_max);
    println!("u32 max: {:>20}", u32_max);
    println!("i64 max: {:>20}", i64_max);
    println!("u64 max: {:>20}", u64_max);
    println!()
}

// ================================================================================================
// 2. floating-point
// ================================================================================================
// f32 by default
// f64 double precision

// ================================================================================================
// 3. Boolean
// ================================================================================================
// true | false

// ================================================================================================
// 4. character
// ================================================================================================
// specify char literal with single quotes

fn rust_char() {
    let rust: char = '🦀';
    println!("char type: {rust}");
    println!();
}

// ================================================================================================
// compound types
// ================================================================================================
// compound types can group multiple values into one type
// 2 primitive types: tuple and array

// ================================================================================================
// 1. tuple
// ================================================================================================
// groups a variety of types into one
// fixed length, cannot grow or shrink after being declared

fn tuple_example() {
    let negative_number = -30000;
    let tuple_string: String = String::from("string");
    let pi: f32 = 3.14;
    let tup: (i32, String, f32) = (negative_number, tuple_string, pi);
    println!("tuple groups together a variaty of types: {:?}", tup);
    println!();
}

// destructure a tuple target
fn destructure_tuple_target() {
    let tup: (u16, f32, u8) = (500, 6.4, 1);
    println!("let tup: (u16, f32, u8) = (500, 6.4, 1);");
    println!("{:?}", tup);
    println!();
    let (x, y, z) = tup;
    println!("destructure_tuple: let (x, y, z) = tup;");
    println!("The floating-point number is: {y}");
    println!("The smaller unsigned number is: {z}");
    println!("The larger unsigned number is: {x}");
    println!();
}

// access a tuple element directly by using a period (.) and indexing into tuple
fn direct_access_tuple_element() {
    let tup: (u16, f32, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    println!("access a tuple element directly: let five_hundred = tup.0");
    println!("{five_hundred}");
    println!();
}
