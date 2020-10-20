fn main() {
    // Create a tuple by writing a comma-separated list of values inside parentheses
    // Each position in the tuple has a type
    // To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value

    let tupTest: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    // destructuring - breaking the single tuple into parts
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // We can access a tuple element directly by using a period (.) followed by the index of the value we want to access
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("Desconstructed tuple: {}; {}; {}", five_hundred, six_point_four, one);
}
