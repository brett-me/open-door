// ================================================================================================
//   CH03-01: VARIABLES AND MUTABALITY
// ================================================================================================

fn main() {
    mutate();
    constant();
    shadow();
}

// ================================================================================================
//  mutating variable
// ================================================================================================
//  by default, variables are mutable
//  explicitly use 'mut' to make mutable

fn mutate() {
    let mut example: u8 = 5;
    println!("mutating variables: orginal value is {}", example);
    example = 10;
    println!("mutating variables: new value is {}", example);
}

// ================================================================================================
//  contstants
// ================================================================================================
// constants are immutable
// can be declared in any scope, including global
// must be an expression

fn constant() {
    const HOURS_IN_A_YEAR: u32 = 365 * 24;
    println!("using a constant: {} hours in a year", HOURS_IN_A_YEAR);
}

// ================================================================================================
//  shadowing
// ================================================================================================
// ownership of a value is dropped if a variables is redeclared and bound to another value
// ownership is also dropped at the end of the scope

fn shadow() {
    let name: String = String::from("initial value: 'joe smoke'");
    println!("{name}");
    let name: u32 = 420; // original value dropped
    println!("pre-nested scope: {}", name);
    {
        let name: bool = true; // dropped again, new type may be invoked
        println!("inside nested scope: {}", name);
    }
    println!("post nested scope: {}", name); // previous iteration of the variable out of scope
}
