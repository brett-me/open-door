// ================================================================================================
//   CH03-03 FUNCTIONS
// ================================================================================================
// functions defined by fn
// scope of function delineated by braces
// functions may be defined anywhere in a scope that can be seen by the caller

// function sans parameters
fn sans_parameters() {
    println!("a function without parameters");
}

// ================================================================================================
//  parameters
// ================================================================================================
// function signatures must declare the type of each parameter
// NB!!! parameters take ownership of variables
// depending on context, a clone, copy or reference may be appropriate

// here, the parameter is a reference, this way, a variable may be passed more than once
fn one_parameter(x: &String) {
    println!("function with a single parameter, {x}");
}

// seperate parameter declarations with a comma
// an expression at the end of a function doesn't require a return prefix or semi-colon
fn no_return_keyword(x: u8, y: u8) -> u8 {
    x + y
}

// ================================================================================================
//  statements and expressions
// ================================================================================================
// statements are instructions that perform an action and do not return a value
// expression evaluate to a resultant value

// ================================================================================================
// functions with return values
// ================================================================================================
// return type must be declared in function signature with ->
// return value synonymous with the value of the final expression (implicit)
// use the 'return' keyword to return early (after a condition for example)

fn passkey() -> u64 {
    3450872235
}
// function above is valid. It simply returns a value as a u64 data type

fn main() {
    println!();
    sans_parameters();
    println!();
    let x: String = String::from("hooray!");
    one_parameter(&x);
    one_parameter(&x); // NB!!! this is only possible because the argument is a reference
    println!();
    let total = no_return_keyword(2, 5);
    println!("2 + 5 = {total}");
    let secret = passkey();
    println!();
    println!("The code is: {secret}");
}
