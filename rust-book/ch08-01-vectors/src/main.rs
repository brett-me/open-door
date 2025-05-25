// ================================================================================================
// CH 08-01 Storing Lists of Values with Vectors
// ================================================================================================
// dynamically sized lists that store values of the same type
// vectors can hold any type

fn main() {
    // annotating is especially important when creating empty vector lists
    // even though vectors are intrinsically dynamic, remember to add mut
    let mut v: Vec<i32> = Vec::new();
    // vector shorthand syntax (macro)
    let mut v_shorthand = vec![1, 2, 3];
    // another
    let mut v_mutable: Vec<String> = vec!["A".into(), "mutable".into(), "vector".into()];

    v.push(1);
    v_shorthand.push(4);

    // reference method 1: NB, will panic if index isn't in scope
    // the reference below is immutable, so we can't push another
    let first: &i32 = &v[0];
    println!("Here, we use an immutable reference, we cant push to this list later: {first}");

    // reference method 2: returns None if index isn't in scope
    let third: Option<&i32> = v_shorthand.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }
    // here, there reference is mutable
    let mutable: &mut String = &mut v_mutable[1];
    println!("Here, we created a mutable reference: {mutable}");

    // pushing after using get, no issues
    v_shorthand.push(5);
    // this works because earlier reference is mutable
    v_mutable.push("hooray".into());
    let sentence = v_mutable.join(" ");
    println!("Using join to print contents of vector: {sentence}");

    println!();
    // using for to iterate through vector
    println!("Using for to iterate through the vector: ");
    for word in &v_mutable {
        println!("{word}");
    }

    println!();
    // change values in vector by deferencing (*)
    println!("Changing the values of the vector via a for loop and dereferencing: ");
    for i in &mut v_shorthand {
        *i *= 2;
        println!("{i}");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // a vector of enum variants enables different storage of different types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}
