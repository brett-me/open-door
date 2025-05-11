// ================================================================================================
//  CH05-03 METHOD SYNTAX
// ================================================================================================
// methods are similar to functions, but are defined in the context of a struct (or enum/trait)
// first parameter is always 'self' (which represents the instance the method is called on)
// Rust makes borrowing implicit for method receivers

// provides method syntax 'rec1.area' (instance.method) [and auto referencing]
// removes having to repeat type self in outer functions
// better organisation of code (group related elements)

// DEFINING METHODS
// ================================================================================================

#[derive(Debug)] // allows debug printing
struct Rectangle {
    width: u32,
    height: u32,
}

// &self uses a reference to the instance (perfect in this 'read' example)
// if writing to instance is required, use &mut self ('self' is rarely passed directly)

// functions defined within impl are associated with the specified struct (below, Rectangle)
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method and attribute names can be the same. They are called differently, see below
    fn width(&self) -> bool {
        self.width > 0 // returns true if width is greater than zero
    }

    // method with more than one parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height // compares instances
    }

    // functions can be defined without self as a parameter
    // often used as constructors
    fn square(size: u32) -> Self {
        // single parameter, definitive quality of square (h=w)
        Self {
            // returns a Self type
            width: size,
            height: size,
        }
    }
}

// also possible to have more than one impl block for a struct
impl Rectangle {
    fn dimensions(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rec2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rec3 = Rectangle {
        width: 60,
        height: 45,
    };
    let sq = Rectangle::square(100); // construct function namespaced by struct
    let rec4 = Rectangle::dimensions(5, 30);

    println!(); // formatting preference
    println!("The area of the rectangle is {} square pixels", rec1.area());

    // below, the method width is called with parenthesis
    if rec1.width() {
        // below, the attribute width is evoked without parenthesis
        println!("The rectangle has a nonzero width of {}", rec1.width);
    }

    println!(); // formatting preference
    println!("Can rec1 hold rec2? {}", rec1.can_hold(&rec2)); // argument is immutable reference
    println!("Can rec1 hold rec3? {}", rec1.can_hold(&rec3)); // rec 2/3 still valid
    println!(); // formatting preference
    println!("A square can be a rectangle, how silly!!! {:?}", sq);
    println!(
        "A more typical example, creating a new rectangle. {:?}",
        rec4
    );
}
