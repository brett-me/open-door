// ===========================================================================================
// part03: Mutability
// green tea coder
// https://www.youtube.com/watch?v=SlRh7O7_1m0&list=PLFdNoRgzggbo6BqJQ6tqk_wvXeI5yfbGA&index=3
// ===========================================================================================

fn main() {
    let x: String = String::from("ten");
    println!("x is: {}", x);
    let x: u8 = 255; // shadowing, previous value of x dropped, type can change
    println!("x has been dropped and now is a number: {}", x);
    let mut y: i32 = 100;
    println!("y is: {}", y);
    y = 50; // possible to change y's value (without declaring let)
    println!("y has now mutated into: {}... but can only be a number", y);
    const KM_TO_M: i32 = 1000; // const is immutable
    println!(
        "{}km is {}m... we used a const in the println! macro",
        y,
        y * KM_TO_M
    );
    println!(); // empty line (design choice)
    println!("i8 limits: {} to {}", i8::MIN, i8::MAX);
    println!("i32 limits: {} to {}", i32::MIN, i32::MAX);
    println!("i64 limits: {} to {}", i64::MIN, i64::MAX);
    println!("isize limits: {} to {}", isize::MIN, isize::MAX);
    println!(
        "usize max: {} (unsigned number cannot be negative)",
        usize::MAX
    );
    println!();
    println!("f32 precision: {} digits", f32::DIGITS);
    println!("f64 precision: {} digits", f64::DIGITS);
    println!();
    let million: i32 = 1_000_000;
    println!("{million:d}");
}
