// This program is better because it uses a bit of structure
// Tuples don't name their elements, so our calculation has become more confusing because we have to index into the parts of the tuple

fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
