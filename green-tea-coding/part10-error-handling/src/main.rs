fn main() {}

fn function_that_panics(num: u8) {
    if num > 5 {
        panic!("Number too large");
    }
    println!("This number is ok");
}

fn function_that_returns_result(num: u8) -> Result<u8, String> {
    if num > u8::MAX / 2 {
        return Err("Number will overflow if doubled".into());
    }
    Ok(num * 2)
}
