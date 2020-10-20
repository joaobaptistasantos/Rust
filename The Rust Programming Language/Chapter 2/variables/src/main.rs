fn main() {
    // By default variables are immutable what means that once a value is bound to a name, you can’t change that value
    // They can be mutable by using mut in front of the variable name
    println!("---------Mutable Variables--------");
    let mut x = 5;
    println!("The value of x is: {}", x);
    
    x = 6;
    println!("The value of x is: {}", x);

    // Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about
    // They are always immutable
    println!("---------Constants--------");
    const MAX_POINTS: u32 = 100_000;
    println!("You can get {} points maximum! ", MAX_POINTS);

    // How Shadowing works
    // The first variable is shadowed by the second, which means that the second variable’s value is what appears when the variable is used
    // By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed
    println!("---------Shadowing--------");
    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of x is: {}", y);

    // Shadowing with variable's types changing
    // The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name
    println!("---------Shadowing w/variable type changing--------");
    let spaces = "   ";
    let spaces = spaces.len();

    println!("You write {} spaces", spaces);
}
