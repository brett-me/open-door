// idiomatic to specify the full path when bringing in structs, enums etc...
use std::collections::HashMap;
use std::fmt::Result;
// create alias for type (effective when types have the same name)
use std::io::Result as IoResult;
// nesting multiple items
use std::{cmp::Ordering, io};

fn main() {
    let mut map = HashMap::new();
    map.insert("name", "brett");
    println!("{:?}", map);
}
