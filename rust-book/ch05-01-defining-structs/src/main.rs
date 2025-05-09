// ================================================================================================
//  CH05-01 DEFINING AND INSTANTIATING STRUCTS
// ================================================================================================
// structs hold multiple related values
// can hold different types
// in structs, each piece of data is named
// use data names to specify or access values of an instance

struct User {
    active: bool,
    username: String, // here, each instance owns its own data
    email: String,    // refs are possible, but require lifetimes
    sign_in_count: u64,
}

// create an instance and specify concrete values for each field
// struct acts a general template

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // use dot notation to access values
    // if the instance is mutable, its possible to change values using dot notation
    // the entire instance must be mutable, not possible to only mark certain fields

    user1.email = String::from("anothermail@example.com");
    println!();
    println!("is active: {}", user1.active);
    println!("username: {}", user1.username);
    println!("email: {}", user1.email);
    println!("sign in count: {}", user1.sign_in_count);

    // call build user function to return an instance
    let user2 = build_user(String::from("user2"), String::from("email@user.com"));

    // creating an instance from an instance
    // NB: this method moves data from one instance to another, making the original invalid
    // NB2: but only for dynamic data, primitive types are copied
    let user3 = User {
        email: String::from("user3@email.com"),
        ..user1 // specifies remaining fields not set must be given same value as reference
    };
    let black = Color(0, 0, 0); // instance of struct tuple
    let origin = Point(0, 0, 0);
    let Point(x, y, z) = origin;
    println!("x is: {}", x);
    let subject = AlwaysEqual; // instance of struct without any fields
}

// a function can return an instance of a struct
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // shorthand, no need to type 'username' twice
        email,    // shorthand
        sign_in_count: 1,
    }
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit like structs without any fields
struct AlwaysEqual; // no curly braces or parenthesis
