// This program introduces DEBUG
// Because println! is a macro we can do many kinds of formatting
// Curly brackets tell println! to use formatting known as Display
// Display - output intended for direct end user consumption
// :? - inside the curly brackets tells println! we want to use an output format called Debug
// Debug - enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);

    // For larger structs we can use {:#?}
    println!("rect1 is {:#?}", rect1);
}
