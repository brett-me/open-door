// ============================================================================================
// CH10-01 Into to Generic Data Types
// ============================================================================================
// Generics replace specific types with a placeholder that represents multiple types
// Used in items like function signatures or structs or enums

fn largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// function below is identical, only difference is parameter type and return type in signature
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// function below uses a generic type in the signature
// this makes the function applicable over different types and reduces code duplication
// requires the use of a trait
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// in struct definitions
struct Point<T> {
    x: T,
    y: T,
}

// generic types over two struct fields
struct Points<T, U> {
    x: T,
    y: U,
}

// in enum definitions
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// in method definitions
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// specifying constraints on generic types
impl Points<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![10, 20, 30, 40, 50];
    let result = largest_number(&number_list);
    println!("The largest number in {:?} is {}", number_list, result);
    println!();

    let char_list = vec!['a', 'e', 'i', 'o', 'u'];
    let result = largest_char(&char_list);
    println!("The largest char in {:?} is {}", char_list, result);
    println!();

    // using generic traits, we can call the same function on different types
    let result = largest(&number_list);
    let result2 = largest(&char_list);
    println!("The largest number in {:?} is {}", number_list, result);
    println!("The largest char in {:?} is {}", char_list, result2);
    println!();

    // using generic traits we can apply differnt types to the same struct
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // use generic over 2 types to use different types for struct fields
    let mix = Points { x: 10, y: 1.0 };
}
