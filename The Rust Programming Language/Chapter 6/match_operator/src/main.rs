// match - allows you to compare a value against a series of patterns and then execute code based on which pattern matches
// patterns - can be made up of literal values, variable names, wildcards, and many other things

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
     value_in_cents(Coin::Quarter(UsState::Alaska));
}

// With an if, the expression needs to return a Boolean value, but here, it can be any type
// match arms - an arm has two parts, a pattern and some code
// => - separates the pattern and the code to run
// When the match expression executes, it compares the resulting value against the pattern of each arm, in order
// If a pattern matches the value, the code associated with that pattern is executed
fn value_in_cents(coin: Coin) -> u8 {
    
    // We add a variable called state to the pattern that matches values of the variant Coin::Quarter
    // When a Coin::Quarter matches, the state variable will bind to the value of that quarter’s state
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
