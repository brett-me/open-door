// =============================================================================
// CH07-02 Defining Modules to Control Scope and Privacy
// =============================================================================

// binary crate saved in src folder 'src/example.rs'
// Test struct, in example.rs, included here
use crate::example::Test;

// tell compiler to include code it finds in example.rs
pub mod example;

fn main() {
    let test = Test {};
    println!("{test:?}... 1... 2...");
}
