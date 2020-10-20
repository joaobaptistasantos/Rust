// Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector
// In Rust memory is managed through a system of ownership with a set of rules that the compiler checks at compile time
// Both the stack and the heap are parts of memory that are available to your code to use at runtime
// stack - (last in, first out) stores values in the order it gets them and removes the values in the opposite order
// All data stored on the stack must have a known, fixed size
// heap - (allocating) stores data with an unknown size at compile time or a size that might change
// When we put data on the heap, we request a certain amount of space then the memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location
// Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
// Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there
// By calling a function, the values passed into the function and the function’s local variables get pushed onto the stack. Once the function is over, those values get popped off the stack

// Ownership Rules
// - Each value in Rust has a variable that’s called its owner;
// - There can only be one owner at a time;
// - When the owner goes out of scope, the value will be dropped.

fn main() {

    // scope - range within a program for which an item is valid
    // A variable is valid from the point at which it’s declared until the end of the current scope

    // We can create a String from a string literal using the from function
    // This type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time
    // string literal - we know the contents at compile time, so the text is hardcoded directly into the final executable -> faster and efficient
    // With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents
    // - The memory must be requested from the memory allocator at runtime.
    // - We need a way of returning this memory to the allocator when we’re done with our String.
    // String::from - requests the memory it needs
    println!("--------Ownership w/String Push---------");    

    let mut s = String::from("hello");

    // In Rust the memory is automatically returned once the variable that owns it goes out of scope
    // Rust calls drop functionautomatically at the closing curly bracket (or another out of scope point)
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // A String is made up of three parts and this group of data is stored on the stack
    // - a pointer to the memory that holds the contents of the string
    // - a length (how much memory, in bytes, the contents of the String is currently using)
    // - a capacity (total amount of memory, in bytes, that the String has received from the allocator)

    // When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack
    // We do not copy the data on the heap that the pointer refers to
    // To ensure memory safety Rust considers s1 to no longer be valid and, therefore, Rust doesn’t need to free anything when s1 goes out of scope
    // Rust will never automatically create “deep” copies of your data -> runtime performance
    println!("--------Ownership w/String Copy---------");
    
    let s1 = String::from("hello");
    let s2 = s1;

    println!("s1 is invalid now, s2 = {}", s2);

    // To deeply copy the heap data of the String, not just the stack data, we can use a common method called clone
    println!("--------Ownership w/String Clone---------");    

    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);
    

    // Types that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make
    // Copy - if a type has the Copy trait, an older variable is still usable after assignment
    // Rust won’t let us annotate a type with the Copy trait if the type, or any of its parts, has implemented the Drop trait
    // Types that are Copy:
    // - All the integer types, such as u32.
    // - The Boolean type, bool, with values true and false.
    // - All the floating point types, such as f64.
    // - The character type, char.
    // - Tuples, if they only contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.
    println!("--------Ownership w/Integers---------");    

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
