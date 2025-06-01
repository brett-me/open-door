// ============================================================================================
// CH 09-03 To Panic! or Not to Panic!
// ============================================================================================
// returning result is a good default choice, calling code decide if recovery is applicable
// prototypes, tests and examples, often better to panic

// ============================================================================================
// Examples, prototype code and test
// ============================================================================================
// unwrap and expect shorthand is faster to implement
// less code make tests / examples clear and easier to follow
// unwrap and expect act as markers in prototype code for future error handling

// ============================================================================================
// Guidelines for error handling
// ============================================================================================
// best to panic when unexpected bad state could occur
// continious running of code requires not being in bad state, panic
// challenging to encode states via types in use, panic

// usually best to return an error if client passes incorrect values...
// if failure is expected, usualy best to return error (eg: parser, file opening, HTTP request)

// ============================================================================================
// Creating types to validate
// ============================================================================================

pub struct Guess {
    // number guessed stored in variable 'value' that holds an i32
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        // generates an instance of a guess if input is valid
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {}
