// ================================================================================================
//   ch05: BORROWING AND REFERENCES
// ================================================================================================
// a value can be used, without moving ownership, through borrowing via referencing (&)
// borrowing via referencing promotes memory safety
// by default, a reference is immutable (use mut to generate mutable reference)

fn main() {
    let x: u8 = 5;
    let r = &x;
    println!("owner x: {x}");
    println!("reference r: {r}");
}
