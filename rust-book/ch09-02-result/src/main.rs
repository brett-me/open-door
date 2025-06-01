// ============================================================================================
// CH 09-02 Recoverable Error with Result
// ============================================================================================

/*

enum Result<T, E> {
    Ok(T),
    Err(E),
}

 */
// Result enum has two variants, Ok and Err
// T and E are generic parameters, T: type and E: error

use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    let greeting_result = File::open("hello.txt");

    // use match expression to hand the Result variants
    let greeting = match greeting_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?} ");
            }
        },
    };

    // without match expression... unwrap_or_else
    let without_match = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // unwrap shortcut (no error message)
    let unwrap = File::open("hello.txt").unwrap();

    // expect shortcut (with error message)
    let expect = File::open("hello.txt").expect("hello.txt should be included in this project");
}

// ============================================================================================
// Propagating Errors
// ============================================================================================
// When a function may fail, possible to return error

fn read_username_from_file() -> Result<String, io::Error> {
    // returns type Result<T, E>, where T is a String and E is io::Error
    // io::Error related to file reading

    let username_file_result = File::open("hello.txt");

    // handle result of opening file
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // handle result of reading String from file
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// using the ? operator
fn shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
// ? placed after a Result value works in a similar way to match
// Error values that are processed via ? go through the From function
// So, error types are automatically linked to the construct used
// ? Operator only works in functions with return type Result

fn chaining() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn even_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
