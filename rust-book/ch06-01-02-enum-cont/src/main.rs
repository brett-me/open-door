// ================================================================================================
// CH 06-01-02 ENUM CONTINUED
// ================================================================================================

// message enum whose variants each store different amounts and types of values
#[derive(Debug)]
enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },    // named fields (like a struct)
    Write(String),              // single String
    ChangeColor(i32, i32, i32), // three i32 values
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
