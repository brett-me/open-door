fn main() {
    let my_fav_book: Book = Book::ZeroToProduction;
    let your_fav_book: Book = Book::TheRustProgrammingLanguage;
    let another_book: Book = Book::RustInAction;
    let custom_book: Book = Book::Other(String::from("Bevy"));

    println!();
    println!("My favourite book is: '{}'", display(&my_fav_book));
    println!("Your favourite book is: '{}'", display(&your_fav_book));
    println!("Hardcoded another book: '{}'", display(&another_book));
    println!("Custom book: '{}'", display(&custom_book));
}

// enums
enum Book {
    TheRustProgrammingLanguage,
    ZeroToProduction,
    RustInAction,
    Other(String), // enums can store data
}

// function matches Book variant to a return String
// Other takes an argument defined by the user
fn title(book: &Book) -> String {
    match book {
        Book::TheRustProgrammingLanguage => "The Rust Programming Language".into(),
        Book::ZeroToProduction => "Zero To Production".into(),
        Book::RustInAction => "Rust In Action".into(),
        Book::Other(title) => title.into(),
    }
}

// function uses match Option (some | none)
// this is how to safely return a none value in Rust
fn author(book: &Book) -> Option<String> {
    match book {
        Book::TheRustProgrammingLanguage => Some("Steve Klabnik".into()),
        Book::ZeroToProduction => Some("Luca Palmieri".into()),
        Book::RustInAction => Some("Tim McNamara".into()),
        Book::Other(_) => None,
    }
}

// uses Some | None again, title and author functions nested here
fn display(book: &Book) -> String {
    let title: String = title(&book);
    match author(&book) {
        Some(author) => format!("{} written by {}", title, author),
        None => format!("{}", title),
    }
}
