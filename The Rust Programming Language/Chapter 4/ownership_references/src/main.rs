// These ampersands are references, and they allow you to refer to some value without taking ownership of it
// The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *
// By have restrictions on mutation Rust can prevent data races at compile time.
// A data race is similar to a race condition and happens when these three behaviors occur:
// - Two or more pointers access the same data at the same time
// - At least one of the pointers is being used to write to the data
// - There’s no mechanism being used to synchronize access to the data

fn main() {
    let s1 = String::from("hello");

    // &s1 syntax lets us create a reference that refers to the value of s1 but does not own it
    // because it does not own it, the value it points to will not be dropped when the reference goes out of scope
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);
}

// Borrowing - having references as function parameters
// & - indicate that the type of the parameter s is a reference
// We’re not allowed to modify something we have a reference to
fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// We had to create a mutable reference with &mut s and accept a mutable reference with some_string: &mut String
// Mutable references restriction - we can have one mutable reference to a particular piece of data in a particular scope
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
