// ================================================================================================
//  CH05-02 EXAMPLE PROGRAMME USING STRUCTS
// ================================================================================================

fn main() {
    println!(); // spacing out the text
    // this code is functional, but it is not clear that width1 and heigh1 are coupled
    let width1 = 30;
    let height1 = 50;
    println!(
        "Eg1: individually defining width1 and height1 (not clear that variables are coupled)- {}",
        area_variables(width1, height1)
    );

    // this code reduces arguments/parameters but is less clear (when indexing into tuple)
    let rect1 = (30, 50);
    println!(
        "Eg2: defining a tuple, rec1. coupling width and height, losing clarity- {}",
        area_tuple(rect1)
    );
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // dbg! returns ownership of expressions value
        height: 50,
    };
    println!(
        "Eg3: defining an instance of a Rectangle, rec2. coupling with clarify- {}",
        // pass reference to rect2 instance
        area_struct(&rect2)
    );
    println!(); // spacing out the text
    println!("rect2 is {:?}", rect2); // use :? to print instance fields
    println!(); // spacing out the text
    dbg!(&rect2); // debugging tool, includes code lines
}

// function eg1: 'decoupled variables'
fn area_variables(width: u32, height: u32) -> u32 {
    width * height
}

// function eg2: rect tuple
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1 // unpack tuple via index
}

// function eg3: rect struct
fn area_struct(rectangle: &Rectangle) -> u32 {
    // borrow instance (fields not copied or moved) so it remains valid in scope of main
    rectangle.width * rectangle.height
}

// struct eg3
#[derive(Debug)] // outer attribute enables println!
struct Rectangle {
    width: u32,
    height: u32,
}
