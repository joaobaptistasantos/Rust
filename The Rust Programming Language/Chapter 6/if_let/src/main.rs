// if let - combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest
// less typing, less indentation, and less boilerplate code

fn main() {
    let some_u8_value = Some(0u8);
    
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }    

    // Instead we could write this in a shorter way using if let
    // We could (optionally) add an else
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("else");
    }
}
