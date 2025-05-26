// ================================================================================================
// CH 08-02 Storing UTF-8 Encoded Text with Strings
// ================================================================================================

fn main() {
    // creating a new, empty String
    let mut s = String::new();

    // different ways to create a String
    let to_string = "initial contents".to_string();
    let from_str = String::from("intial contents");
    let string_into: String = "initial contents".into();

    // ========================================================================================
    // updating a String
    // ========================================================================================
    // appending a string slice to a String using the push_str method
    let mut s = String::from("foo");
    s.push_str("bar"); // the parameter "bar" goes out of scope after appending to s

    // define a variable if we want to preserve validity
    let mut s1 = "foo".to_string();
    let s2 = "bar";
    s1.push_str(s2); // push_str to append a string slice
    println!("s2 is still valid... {}", s2);

    let mut lol: String = "lo".into();
    lol.push('l'); // use push to append a char

    // using the + operator to combine two string values into a new String value
    let hello = String::from("hello");
    let world = String::from("world");
    let hello_world = hello + &world; // variable hello is no longer valid, value moved
    println!("{hello_world}");

    // format! easily combines multiple Strings
    let tic = "tic".to_string();
    let tac: String = "tac".into();
    let toe = String::from("toe");
    // format!() works like println!, but returns a String (rather than print to screen)
    let tic_tac_toe = format!("{tic}-{tac}-{toe}");

    // ========================================================================================
    // Generally, complicated to index into a String
    // Why, because a letter might take more than 1 byte to incode to UTF-8
    // Creating a string slice with ranges might cause the programme to crash
    // ========================================================================================

    // use chars() to be more explicit
    for c in "Зд".chars() {
        println!("{c}");
    }

    // or depending on domain, bytes may be approapiate
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
