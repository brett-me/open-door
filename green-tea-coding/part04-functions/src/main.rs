// ===========================================================================================
// part04: Functions
// green tea coder
// https://www.youtube.com/watch?v=SlRh7O7_1m0&list=PLFdNoRgzggbo6BqJQ6tqk_wvXeI5yfbGA&index=3
// ===========================================================================================

fn show_number(num: i32) {
    println!("The number is {}", num);
}

fn add_numbers(a: &i32, b: &i32) -> i32 {
    a + b
}

fn main() {
    show_number(10);
    let a: i32 = 1;
    let b: i32 = 4;
    println!("{} + {} = {}", a, b, add_numbers(&a, &b));
    println!(); // empty line (design choice)
    println!("{a} + {b} = {}", a + b); // also possible to pass operations directly into println! macros
}
