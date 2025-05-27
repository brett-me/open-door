// ================================================================================================
// CH 08-03 Storing Keys with Associated Values in Hash Maps
// ================================================================================================
// hash maps store data on the heap and are homogeneous (keys must be all the same, values too)
// hashmap types are expensive, but offer better security (for performance, use hasher type)

use std::collections::HashMap;

fn main() {
    // creating a new hash map
    let mut pl_points = HashMap::new();

    pl_points.insert(String::from("Liverpool"), 84);
    pl_points.insert(String::from("Arsenal"), 76);

    // accessing points for liverpool stored in the hash map
    let team_name = String::from("Liverpool");
    let points = pl_points.get(&team_name).copied().unwrap_or(0);
    // get method returns an Option<&V>, None is returned if no value for key
    // copied to get Option<V> rather than <&V>
    // unwrap_or to set points to given value if no key
    println!("{points}");

    // iterating over each key-value pair
    for (k, v) in &pl_points {
        println!("{k}: {v}");
    }

    let field_name = String::from("colour");
    let field_value = String::from("blue");
    let mut colours = HashMap::new();
    colours.insert(&field_name, &field_value); // Ownership Moved to HashMap, no longer valid
    println!("{field_name}: {field_value}");
    // references can be passed into the hashmap,
    // but the hashmap references only valid for as long as the variables are

    // overwriting a value
    pl_points.insert(String::from("Arsenal"), 80);
    // this over-rides the previous value (76) associated with the key "Arsenal"

    // using the entry method to only insert if the key does no already have a value
    pl_points.entry(String::from("Arsenal")).or_insert(90);
    pl_points.entry(String::from("Man City")).or_insert(75);

    // Man City added to Hash Map (doesn't exist before statement)
    // Arsenal points not changed (arsenal already existed)
    for (team, points) in &pl_points {
        println!("{team}: {points}");
    }

    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
    // iterating over a hash map happens in an arbitary order
    // split_whitespace method returns an iterator over subslices seperated by whitespace
    // or_insert returns a mutable reference to the value for the specified key
    // mutable reference stored in variable count (deref)
}
