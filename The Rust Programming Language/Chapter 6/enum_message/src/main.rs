// Quit has no data associated with it at all
//struct QuitMessage; // unit struct

// Move includes an anonymous struct inside it
//struct MoveMessage {
//    x: i32,
//    y: i32,
//}

// Write includes a single String
//struct WriteMessage(String); // tuple struct

// ChangeColor includes three i32 values
//struct ChangeColorMessage(i32, i32, i32); // tuple struct

// The previous structs could hold the same data that the preceding enum variants hold
// But if we used the different structs, which each have their own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum
// As on structs, on enums we are able to define methods using impl
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
