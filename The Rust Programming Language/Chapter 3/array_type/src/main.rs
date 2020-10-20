fn main() {
    // Every element of an array must have the same type
    // In Rust, arrays have a fixed length, like tuples
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    // Arrays are useful when you want your data allocated on the stack rather than the heap
    // Another way to initialize an array is by using square brackets, and within the brackets include the type of each element, a semicolon, and then the number of elements in the array
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // If we want to create an array that contains the same value for each element, we can specify the initial value, followed by a semicolon, and then the length of the array in square brackets
    let b = [3; 5];

    // Accessing an element by index
    let first = a[0];
    let second = a[1];

    println!("First element of 'a' array: {}", first);
    println!("Second element of 'a' array: {}", second);
}
