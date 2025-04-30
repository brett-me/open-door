// ================================================================================================
//  EX003: FORMATTED PRINTING [fmt::Display, {} as marker]
// ================================================================================================
// for more, see: https://doc.rust-lang.org/std/fmt/

// printing is handles by a series of macros, some of which are:
//    - format! :write formatted text to a string
//    - print! :same as format! but text is printed to the console
//    - println! :same as print! but a new line is appended
//    - eprint! :same as print! but text is printed to the standard error
//    - eprintln! :same as eprint! but a new line is appended

fn main() {
    basic_print(); // argument passed to braces
    positional_arguments(); // indexing arguments into braces
    named_arguments(); // passing named arguments into braces
    variable_arguments(); // passing variables as arguments into braces
    variables_in_braces(); // variables directly in braces (expressions not valid)
    base_formatting(); // base 2 (binary), 8 (octal), 16 (hexadecimal)
    right_justify();
    padding();
    decimal_places();
}

// ================================================================================================
// example 1: passing arguments into braces
// ================================================================================================

fn basic_print() {
    println!("example 1: {} days", 31);
    // at a minimum, the print macros takes a format string argument
    // here, the println! macros takes two arguments
    // the second argument is passed to the braces and will be stringified
}

// ================================================================================================
// example 2: indexing arguments into braces
// ================================================================================================

fn positional_arguments() {
    println!(
        "example 2: this is '{1}'. this is '{0}', this is '{1}' again",
        "zero", // 2nd argument (index 0)
        "one"   // 3rd argument (index 1)
                // specifying an integer inside braces determines which argument will be passed
                // argument are indexed starting from 0 after the format string
    );
}

// ================================================================================================
// example 3: passing named arguments into braces
// ================================================================================================

fn named_arguments() {
    println!(
        "example 3: {subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
}

// ================================================================================================
// example 4: passing variables as arguments into braces
// ================================================================================================

fn variable_arguments() {
    let x: u8 = 5;
    let y: u8 = 3;
    println!("example 4: {} + {} = {}", x, y, x + y);
    // an expression argument (index 3) can be passed into braces
}

// ================================================================================================
// example 5: arguments directly in braces
// ================================================================================================

fn variables_in_braces() {
    let a: u8 = 2;
    let b: u8 = 3;
    let c: u8 = a + b;
    println!("example 5a: {a} + {b} = braces can't calculate expressions");
    println!("example 5b: {a} + {b} = {c} here 'c' is a variable with an expression bound to it");
}

// ================================================================================================
// example 6: formatting (base 2, 8, 16)
// ================================================================================================
// formatting invoked by specifing the format character after ':'

fn base_formatting() {
    let base_10: u8 = 255;
    println!("example 6a: 255 expressed in binary (base 2) is {base_10:b}");
    println!("example 6b: 255 expressed in octal (base 8) is {base_10:o}");
    println!("example 6c: 255 expressed in hexadecimal (base 16) is {base_10:x}");
}

// ================================================================================================
// example 7: right justify
// ================================================================================================
// format character for right justify is >
// string will start on specified index (ie: >5 amounts to 4 blank spaces before text)

fn right_justify() {
    println!("example 7: {number:>5}", number = 100);
}

// ================================================================================================
// example 8: padding
// ================================================================================================

fn padding() {
    println!("example 8a: {:0>5}", 1); // pads argument with four zeros to the left
    println!("example 8b: {number:9<3}", number = 1); // pads digit with two nines to the right
    let number: u8 = 1;
    println!("example 8c: {number:0<width$}", width = 5); // notice $
}

// ================================================================================================
// example 9: decimal places (rounding)
// ================================================================================================

fn decimal_places() {
    println!("example 9: {pi:.2}", pi = 3.141592)
}
