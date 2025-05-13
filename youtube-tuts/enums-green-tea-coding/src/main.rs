// ================================================================================================
// Enums: Green Tea Coding
// https://www.youtube.com/watch?v=-nFDSDE_2Nw
// ================================================================================================

#[allow(dead_code)]

enum Book {
    TheRustProgrammingLanguage,
    ZeroToProduction,
    RustInAction,
    Other(String),
}

fn title(book: &Book) -> String {
    match book {
        Book::TheRustProgrammingLanguage => "The Rust Programming Language".into(),
        Book::ZeroToProduction => "Zero To Production".into(),
        Book::RustInAction => "Rust In Action".into(),
        Book::Other(title) => title.into(),
    }
}

fn author(book: &Book) -> Option<String> {
    match book {
        Book::TheRustProgrammingLanguage => Some("Steve Klabnik".into()),
        Book::ZeroToProduction => Some("Lica Palmiere".into()),
        Book::RustInAction => Some("Tim McNamara".into()),
        Book::Other(_) => None,
    }
}

fn display(book: &Book) -> String {
    let title: String = title(&book);
    match author(&book) {
        Some(author) => format!("{} written by {}", title, author),
        None => format!("{}", title),
    }
}

fn main() {
    let my_favourite_book = Book::ZeroToProduction;
    let your_favourite_book = Book::Other("Atomics and Locks".into());
    println!("My favourite book is: '{}'", display(&my_favourite_book));
    println!(
        "Your favourite book is: '{}'",
        display(&your_favourite_book)
    );
}
