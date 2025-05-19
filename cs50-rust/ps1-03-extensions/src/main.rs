use std::io::stdin;

fn main() {
    println!("File name:");
    let file_name: String = get_input();
    println!("{}", media_type(&file_name));
}

fn get_input() -> String {
    let mut file_name = String::new();
    stdin()
        .read_line(&mut file_name)
        .expect("String input: 'filename.extension'");
    file_name.trim().to_lowercase()
}

fn media_type(file_name: &str) -> &str {
    match file_name {
        s if s.ends_with(".gif") => "image/gif",
        s if s.ends_with(".jpg") => "image/jpg",
        s if s.ends_with(".jpeg") => "image/jpeg",
        s if s.ends_with(".png") => "image/png",
        s if s.ends_with(".pdf") => "application/pdf",
        s if s.ends_with(".txt") => "text/plain",
        s if s.ends_with(".zip") => "application/zip",
        _ => "application/octet-stream",
    }
}
