// Methods are declared within th fn keyword and their name, they can have parameters and a return value, and they contain some code that is run when they're called from somewhere else
// Methods are defined within the context of a struct and their first parameter is always self - represents the instance of the struct the method is being called on
// Rust has a feature called automatic referencing and dereferencing (automatically adds in &, &mut, or * so object matches the signature of the method)

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// We start an implementation block to define the function within the context of Rectangle
// self - Rust recognizes that the type self is from the type of data inside the implementation block
// Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter
// Methods can take multiple parameters

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// implementation blocks allow us to define functions within impl blocks that don’t take self as a parameter - associated functions
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);

    println!("Square Rectangle: {:#?}", sq);
}
