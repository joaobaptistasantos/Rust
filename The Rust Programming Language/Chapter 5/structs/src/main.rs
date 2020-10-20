// The pieces of a struct can be different types
// We'll name each piece of data
// struct - keyword to define a struct
// structs can store references to data owned by something else, but to do so requires the use of lifetimes

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple structs - added meaning the struct name provides but don’t have names associated with their fields. They just have types of the fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs - structs that don't have any fields, behave similarly to (), the unit type

fn main() {
    // To use a struct, we define it, and after that we create an instance of that struct by specifying concrete values for each of the fields
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // To get a specific value from a struct, we use dot notation
    println!("User 1: {}", user1.username);

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("Email User 2: {}", user2.email);

    // If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field
    user2.email = String::from("anotheremail@example.com");

    println!("Email User 2: {}", user2.email);

    println!("Username User 1: {}", user1.username);

    // Using struct update syntax we can create a new User instance using another User info
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("Username User 3: {}", user3.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// We can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance
// Once the parameter names and the struct field names are exactly the same we can use the field init shorthand syntax
fn build_user(email: String, username: String) -> User {
    User {
        //email: email,
        //username: username,
        email,
	username,
	active: true,
        sign_in_count: 1,
    }
}
