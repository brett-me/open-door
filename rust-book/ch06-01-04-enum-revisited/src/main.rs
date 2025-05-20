// enums group sets of related values

// generate custom enum data type called IpAddrKind with 2 variants
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    // create instances of enum variants
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}", four);
    println!("{:?}", six);
    // enum variate as function argument
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    // create instance of enum variant with associated data
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);
    // create instance of Message enum, Write Variant with associated "Hello" String
    let m = Message::Write(String::from("Hello"));
    // use Message method
    m.call();
    // using enum Option to work with None values
    // useful when a value can be something or it could be nothing
    let number: i32 = 5;
    let some_number: Option<i32> = Some(5);
    let absent_number: Option<i32> = None;
    println!("{:?}, {:?}, {:?}", number, some_number, absent_number);
}

// use enum as a function parameter
fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind);
}

// various types of data can be associated with enums
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// enum with variants that store different amounts and types of values
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// define method on enum
impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}
