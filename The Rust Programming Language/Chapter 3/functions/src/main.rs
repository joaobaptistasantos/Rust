// We can create functions befor or after main
// Functions definitions in Rust start with fn and have a set of parentheses after the function name
// parameters - special variables that are part of a function’s signature
// declare the type of each parameter
// They can return values to the code that calls them
// Return values need only the declaration of their type after an arrow '->'

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("--------Function Parameters---------");

    another_function();

    another_function2(5);

    another_function3(5, 6);

    // Functions definitions are also statements, statements do not return values
    // But we cannot assign a let statetment to another variable
    // Expressions evaluate to something, they can be part of statements, calling a function or a macro is an expressions, the block used to create new scopes {} is an expression
    println!("--------Function Bodies Contain Statements and Expressions--------");
    
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);


    println!("--------Functions with Return Values--------");
    
    let z = five();

    println!("The value of z is: {}", z);

    let r = plus_one(5);

    println!("The value of r is: {}", r);
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function3(x: i32, y: i32) {
    println!("The value of a is: {}", x);
    println!("The value of b is: {}", y);
}
