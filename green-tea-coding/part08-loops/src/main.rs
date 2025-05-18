fn main() {
    println!();
    let words: [String; 3] = ["this".into(), "is".into(), "sparta!".into()];
    println!("{:?}", words);

    let sentence = words.join(" ");
    println!("{}", sentence);

    let shout: Vec<String> = sentence
        .to_uppercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    println!("{:?}", shout);

    println!();
    let sum: i32 = (0..=20).step_by(2).sum();
    println!("{}", sum);
}
