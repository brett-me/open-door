fn main() {
    println!();
    // arrays: fixed sized, homogenous elements, stored on stack
    let mut example_array: [u8; 3] = [10, 20, 30];
    println!("array: {:?}", example_array);
    // we can change the value of an element in an array, but we can't add or subtract elements
    example_array[0] = 15;
    println!(" array- edit element: {:?}", example_array);

    //vectors: dynamically sized
    let mut example_vec: Vec<u8> = vec![10, 20, 30];
    println!("vector: {:?}", example_vec);
    example_vec.push(40);
    println!("vector- push element: {:?}", example_vec);
    println!("sum of vector elements = {}", sum_elements(&example_vec));
    // notice that a slice of the vector is passed as an argument into the function
}

// remember, function parameter should be a reference if the function does not modify the parameter
fn sum_elements(a: &[u8]) -> u8 {
    let mut sum = 0;
    for i in a {
        sum += i;
    }
    sum
}
