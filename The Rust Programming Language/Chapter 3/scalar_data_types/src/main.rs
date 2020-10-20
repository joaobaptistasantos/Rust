fn main() {
    // Integer
    // Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive
    // Unsigned variants can store numbers from 0 to 2n - 1
    // isize and usize types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture
    // isize or usize is when indexing some sort of collection
    // Note: i32: this type is generally the fastest, even on 64-bit systems

    // Floating-Point
    // Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size
    // The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // Character
    // char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII
    // Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
