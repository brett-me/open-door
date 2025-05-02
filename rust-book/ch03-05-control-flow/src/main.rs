// ================================================================================================
//   CH03-05 CONTROL FLOW
// ================================================================================================

fn main() {
    println!(); // empty line to seperate compiler text from programme text
    println!("{}", basic_if_expression(100)); // argument positive, true printed
    println!("{}", basic_if_expression(-100)); // argument negative, false printed
    println!();
    multiple_conditions_with_else_if();
    println!();
    using_if_in_a_let_statement();
    println!();
    cute_cat();
    println!();
    loop_example();
    loop_labels();
    println!();
    while_conditional(10);
    println!();
    safe_collection_loop();
    println!();
    safe_launch();
}
// ================================================================================================
// if expressions
// ================================================================================================
// if statements must evaluate to either true or false

fn basic_if_expression(n: i32) -> bool {
    // returns true if argument is positive, false if negative
    if n > 0 {
        return true;
    } else {
        return false;
    }
}
// ================================================================================================
// handling multiple conditions with else if
// ================================================================================================
// Rust will exit the loop once the first true condition is me (it wont check the rest)
// limit the number of else if statments (many may indicate that using 'match' is a better tool)

fn multiple_conditions_with_else_if() {
    let number: u8 = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3"); // loop will exit after this line
    } else if number % 2 == 0 {
        println!("number is divisible by 2"); // even though this is true, this code is not reached
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
// ================================================================================================
// using if in a let statement
// ================================================================================================
// because if is an expression it can be used on the right side of a let statement
// outcome variables must be of the same type

fn using_if_in_a_let_statement() {
    let condition: bool = true;
    let number = if condition { 5 } else { 6 }; // note that 5 and 6 are both the same type
    println!("The value of number is: {number}");
}

fn cute_cat() {
    let mut is_cute: bool = true;
    let mut leroy = if is_cute { "is cute" } else { "is fugly" };
    println!("Leroy {leroy}");
    // for this example, is_cute and leroy made mut to show both outcomes simulataneously
    // another approach would be to redeclare both is_cute and leroy with a preceeding 'let'
    is_cute = false;
    leroy = if is_cute { "is cute" } else { "is fugly" };
    println!("Leroy {leroy}");
}

// ================================================================================================
// repition with loops
// ================================================================================================
// 1. loop, 2. while and 3. for

// ================================================================================================
// repeating code with LOOP
// ================================================================================================
// the loop keyword tells Rust to execute a block of code over and over again forever
// can be terminated with 'break' expression (usually combined with if condition)
// in addition to 'break', 'continue' may also be used
// NB! with loop, break can return a value

fn loop_example() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter - 10; // break returns an expression, so counter can be manipulated here
        }
    };
    println!("The result is: {result}");
}

// ================================================================================================
// loop labels
// ================================================================================================
// break and continue apply to the innermost loop at that point
// alternatively, loop lables can be used to direct break and continue
// loop labels begin with a single quote

fn loop_labels() {
    let mut count = 0;
    // ================================ outer-loop ====================================================
    'counting_up: loop {
        println!();
        println!("count = {count}");
        let mut remaining = 10;
        // ==== inner-loop ===========================
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        // ==== inner-loop ============================
        count += 1;
    }
    // ================================ outer-loop ====================================================
    println!("End count = {count}");
}

// ================================================================================================
// conditional loops with while
// ================================================================================================

fn while_conditional(mut n: u8) {
    while n != 0 {
        println!("{n}");
        n -= 1;
    }
    println!("Liftoff!");
}

// ================================================================================================
// looping through a collection using 'for'
// ================================================================================================

fn safe_collection_loop() {
    let collection: [u8; 5] = [10, 20, 30, 40, 50];
    for i in collection {
        println!("the value is: {i}");
    }
}

// ================================================================================================
// using for to loop through a range (safer than while)
// ================================================================================================

fn safe_launch() {
    // (from..to[exclusive]) is standard range format. Also, (from..=to[inclusive])
    for i in (1..4).rev() {
        println!("{i}");
    }
    println!("liftoff!");
}
