// ================================================================================================
//  CH06-01 DEFINING AN ENUM
// ================================================================================================
// enums provide a way of saying a value is one of a possible set of values
// eg: a rectangle is one of a possible set of shapes that includes circle and triangle
// possibilities encoded as an enum
// an enum value can only be one of it's variants

// generate a custom enum data type named 'IpAddr'
// technically, part of the standard library, but no conflict here because library not in scope
enum IpAddr {
    V4(u8, u8, u8, u8), // Number type data attached to variant
    V6(String),         // String type data attached to variant
}

fn route(ip_kind: IpAddrKind) {}

fn main() {
    // create instances of the two variants of IpAddrKind
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
