// ================================================================================================
// CH 06-01-03 OPTION ENUM
// ================================================================================================
// Option enum is defined by the standard library
// encodes a scenario which a value could be something, or it could be nothing
// Rust does not have a null feature
// Rust uses the enum option function that considers a value as being absent or present

/* std library definition of enum Option
enum Option<T> {
    None,
    Some(T),
}
No need to bring it into scope explicitly
*/

// <T> represents a generic type parameter (ie, any type)
// Type passed to some will change enum Option type

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');
    let some_string = Some(String::from("This is a dynmaic String"));

    let absent_number: Option<i32> = None; // None type requires annotation
    let absent_char: Option<char> = None;
    let absent_string: Option<String> = None;
}
