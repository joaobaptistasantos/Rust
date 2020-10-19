use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate a secret number using the mehod gen_range() from 'rand' crate
    // thread:rng - local to the current thread of execution and seeded by the operating system
    // gen_range - method on the random number generator, takes two numbers as arguments and generates a random number between them (inclusive on the lower bound and exclusive on the upper bound)
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    // loop - creates an infinite loop
    loop {
    	println!("Please input your guess.");

    	// In Rust, variables are immutable by default
    	// let statement - used to create a variable
        // mut - make a variable mutable
    	// String::new - function that returns a new instance of a String (an UTF-8 encoded bit of text)
    	let mut guess = String::new();

    	// The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for the termina
    	// read_line method to get input from the user. It takes a string as an argument that need to be mutable so the content can be replaced by the user input
    	// & - indicates that this argument is a reference
    	// read_line returns an io::Result - an enumeration where the variantes are Ok (indicates the operation was successfull) and Err (means the operation failed)
    	io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

    	// i32 - a 32-bit number
    	// u32 - an unsigned 32-bit number
    	// i64 - a 64-bit number
    	// Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables
    	// trim - will eliminate any whitespace at the beginning and end
    	// u32 - can contain only numerical characters, the user must press enter to satisfy read_line
    	// parse - can parse a variety of number types, we need to tell Rust the exact number type we want 
    	// switching from an expect call to a match expression is how you generally move from crashing on an error to handling the error
	let guess: u32 = match guess.trim().parse() {
	    Ok(num) => num,
            Err(_) => continue,
	};

    	println!("You guessed: {}", guess);

    	// Ordering - enum with variants 'Less', 'Greater' and 'Equal' (possible outcomer when we compare two values)
    	// cmp - method that compares two values and returnas a variant of Ordering
    	// match - expression made up of arms - a pattern and the code that should be run if the value given to the beggining of the match expression fits that arm's pattern
    	match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
		println!("You win!");
		break;
	    }
    	}
    }
}
