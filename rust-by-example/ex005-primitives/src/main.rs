// ================================================================================================
//  EX005: Primitives
// ================================================================================================
// SCALAR TYPES
// Signed integers: i8 through isize
// Unsigned integers: u8 through usize
// Floating point: f32, f64
// char: Unicode
// bool: true or false
// unit type: () empty tuple

// COMPOUND TYPES
// Arrays [1, 2, 3] ... fixed size homogenous
// Tuples (1, true) ... fixed size heterogenous

#![allow(dead_code)]
fn main() {
    // variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0; // regular annotation
    let an_integer = 5i32; // suffix annotation

    // or a default will be used
    let default_float = 3.0; // f64
    let default_integer = 7; // i32

    // a type can also be inferred from context
    let mut inferred_type = 12; // type i64 is inferred from...
    inferred_type = 4294967296000000i64;

    // a mutable variable's value can change (but not type)
    let mut mutable = 12; // mutable i32
    mutable = 21;

    // variables can be overwritten with shaddowing
    let mutable = true;

    /* Compound types - Array and Tuple */
    // array signature consists of Type T and length as [T; 10]
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // tuple is a collection of values of different types
    // and is constructed using parenthesis ()
    let my_tuple = (5u32, 1u8, true, -5.04f32);
}
