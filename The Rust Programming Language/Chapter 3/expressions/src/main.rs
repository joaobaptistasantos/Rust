fn main() {
    let number = 3;

    // if expressions start with the keyword if, which is followed by a condition
    // Blocks of code associated with the conditions in if expressions are sometimes called arms, just like the arms in match expressions
    println!("--------If Condition--------");

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Multiple  conditions by combining if and else in an else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if is an expression, we can use it on the right side of a let statement
    let condition = true;
    let number2 = if condition { 5 } else { 6 };

    println!("The value of number2 is: {}", number2);

    // Rust has three kinds of loops: loop, while, and for
    // loop - execute a block of code over and over again forever or until we explicitly tell it to stop
    // One of the uses of a loop is to retry an operation we know might fail, such as checking whether a thread has completed its job
    // To do that we can add the value we want returned after the break expression you use to stop the loop
    println!("--------Loop Condition w/Returning Value--------");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // while - evaluate a condition within a loop
    // While the condition is true, the loop runs
    println!("--------While Condition--------");

    let mut number3 = 3;

    while number3 != 0 {
        println!("{}!", number3);

        number3 -= 1;
    }

    println!("--------While Condition on a Collection--------");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    println!("LIFTOFF!!!");

    println!("--------For Condition--------");

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
