// ================================================================================================
//   CH06-00-recap
// ================================================================================================
// to enumerate is to mention one-by-one
// use enums to list possible varients of a value (eg: enum Pet could contain {dog, cat, fish})
// each variant in an enum are of the same type

enum Pet {
    Dog(String), // attaching data to a variant
    Cat(String), // each variant can have different types of data (or none)
    Fish(String),
}

// functions can take an enum variant as a parameter
fn speak(pet_kind: &Pet) -> String {
    match pet_kind {
        Pet::Dog(_name) => String::from("woof"),
        Pet::Cat(_name) => String::from("meow"),
        Pet::Fish(_name) => String::from("blob blob"),
    }
}

impl Pet {
    fn movement(pet_kind: Pet) -> String {
        match pet_kind {
            Pet::Dog(_name) => String::from("walk"),
            Pet::Cat(_name) => String::from("climb"),
            Pet::Fish(_name) => String::from("swim"),
        }
    }
}

fn main() {
    let leroy = Pet::Cat(String::from("leroy")); // one way to create an instance of a variant
    let yambo = Pet::Dog(String::from("yambo")); // String follows the same pattern
    let nemo = Pet::Fish(String::from("nemo"));

    speak(&leroy); // passing an enum variant as an argument to a function
    speak(&yambo);
    println!("{}", speak(&nemo));

    println!("{}", Pet::movement(leroy));
    Pet::movement(yambo);
    Pet::movement(nemo);
}
