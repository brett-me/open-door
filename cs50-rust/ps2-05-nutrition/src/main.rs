use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let item = get_item();
    let calories = calories(&item);
    if calories == 0 {
        println!("{item} is not in database");
    } else {
        println!("{calories}");
    }
}

fn get_item() -> String {
    print!("Item: ");
    io::stdout().flush().unwrap();

    let mut item = String::new();
    io::stdin()
        .read_line(&mut item)
        .expect("Text input as String");
    item = item.trim().to_lowercase();
    item
}

fn calories(item: &str) -> u8 {
    let fruits = HashMap::from([
        ("apple", 130),
        ("avocado", 50),
        ("banana", 110),
        ("cantaloupe", 50),
        ("grapefruit", 60),
        ("grapes", 90),
        ("honeydew melon", 50),
        ("kiwifruit", 90),
        ("lemon", 15),
        ("lime", 20),
        ("nectarine", 60),
        ("orange", 80),
        ("peach", 60),
        ("pear", 100),
        ("pineapple", 50),
        ("plums", 70),
        ("strawberries", 50),
        ("sweet cherries", 100),
        ("tangerine", 50),
        ("watermelon", 80),
    ]);

    fruits.get(&item).copied().unwrap_or(0)
}
