// ================================================================================================
//  EX004: Debug
// ================================================================================================
// All types can derive (automatically create) the fmt::Debug implementation.
// This is not true of fmt:Display, which must be manually implemented.

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
    // Debug Print
    println!("{:?}", peter);
}
