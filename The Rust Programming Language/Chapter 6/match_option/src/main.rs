fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        // _ => (), // we could add this _ pattern that will match any value
    }
}

fn main() {
    let five = Some(5);
    
    println!("Number: {:#?}", five);

    let six = plus_one(five);

    println!("Number plus one: {:#?}", six);
    
    let none = plus_one(None);

    println!("None plus one: {:#?}", none);
}
