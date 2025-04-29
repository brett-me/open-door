// ================================================================================================
//   ch03: FUNCTIONS
// ================================================================================================
// main function is mandatory
// hoisting: functions can appear anywhere, below or above the main function (valid in rust)

fn main() {
    basic_print(); // function passed into main
    take_number_argument(42); // function takes argument
    human_id("John Cena", 40, 1.92, true); // multiple arguments (different types)
    // bind function return to variable and print variable
    let amount_due = total_price(99, 10);
    println!("Total: R{amount_due} (x here is a printed variable, which was bound to a function)");
    // pass function directly to print (bypass variable declaration)
    println!(
        "Passing function directly to print (Total: R{})",
        total_price(10, 10)
    );
    // directly binding a return value to a variable (no function)
    let x: i32 = {
        let y = 1;
        let z = 2;
        y + z // notice, no return keyword, no semi-colon (can be added)
    };
    println!("1 + 2 = {x} (x in this example is a variable directly bound to a return value)");
    // also possible to print result of expression directly
    println!(
        "10 + 20 = {} (x in this example is calculated directly in the println!",
        10 + 20
    );
    // variable as arguments
    let weight: f64 = 100.00;
    let height: f64 = 1.87;
    println!("BMI = {:.2}", calculate_bmi(weight, height)); // use :.2 to format floats
    let random_float: f32 = 1.2345;
    println!("{random_float:.2}"); // :. format float works directly after variable in braces
}

fn basic_print() {
    println!("hello_world");
}

fn take_number_argument(number: i32) {
    println!("This number ({number}) was passed through the functiona as an argument");
}

fn human_id(name: &str, age: u8, height: f32, is_registered: bool) {
    println!("name: {name}, age: {age} years, height: {height}m, is registered: {is_registered}");
}

// ================================================================================================
//   expressions
// ================================================================================================
// returns a value

fn total_price(unit_price: u32, quantity: u32) -> u32 {
    unit_price * quantity // notice, no return word, not semi-colon
}

fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}
// ================================================================================================
//   statements
// ================================================================================================
// does not return a value
// mostly ends with ;
// NB a statement cannot be bound to another statement (eg: let x = let y = 10 produce an error)
