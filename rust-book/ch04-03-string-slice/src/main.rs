// ================================================================================================
//  CH04-03 slice type
// ================================================================================================
// slices are a contiguius seqeunce of elements
// a slice is a kind of reference and does not have ownership

fn main() {
    println!();
    let sentence = String::from("Learning how to code in Rust");
    println!("{}", first_word(&sentence));
}

// example
// ================================================================================================
// return the first word in a given string

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // convert to an array of bytes (allows checking of elements, 1-by-1)
    // iter returns each element in a collection
    // enumerate wraps the result of iter and returns each element as part of a tuple
    // tuple includes index and a reference to the element (i, &item)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
