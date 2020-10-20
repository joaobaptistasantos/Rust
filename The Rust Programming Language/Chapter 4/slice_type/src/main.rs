fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    println!("the first word is: {}", word);

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // string slice - reference to part of a String
    // to create slices - range within brackets by specifying [starting_index..ending_index], where starting_index is the first position in the slice and ending_index is one more than the last position in the slice
    // Internally, the slice data structure stores the starting position and the length of the slice
    // .. - Rust range syntax
    // String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // &s[..2]
    // &s[3..]
    // &s[..]
}

fn first_word(s: &str) -> &str {
    // Convert String to an array of bytes using the as_bytes - because we need to go through the String element by element and check whether a value is a space
    let bytes = s.as_bytes();
    
    // iter - create an iterator over the array of bytes
    // iter - method that returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead
    // first element - index
    // second element - reference to the element
    for (i, &item) in bytes.iter().enumerate() {

       // Search for the byte that represents the space by using the byte literal syntax
        if item == b' ' {
            // If we find a space, we return the position
            return &s[0..i];
        }
    }

    // Otherwise, we return the length of the string by using s.len()
    &s[..]
}
