// Enums allow us to define a type by enumerating its possible variants
// Enum values can only be one of its variants

// Less concise way
enum IpAddrKind {
    V4,
    V6,
}

//struct IpAddr {
//    kind: IpAddrKind,
//    address: String,
//}

// We can put data directly into each enum variant
// Each variant can have different types and amounts of associated data
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Or even use a struct to represent a variant of an enum
//struct Ipv4Addr {
    // --snip--
//}

//struct Ipv6Addr {
    // --snip--
//}

//enum IpAddr {
//    V4(Ipv4Addr),
//    V6(Ipv6Addr),
//}

fn main() {
    // Create instances of each of the two variants of IpAddrKind
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Send enumeration as parameter
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Instantiate a struct with a field kind that is of type IpAddrKind
    //let home = IpAddr {
    //    kind: IpAddrKind::V4,
    //    address: String::from("127.0.0.1"),
    //};
    //
    //let loopback = IpAddr {
    //    kind: IpAddrKind::V6,
    //    address: String::from("::1"),
    //};

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

}

// Function that takes any IpAddrKind
fn route(ip_kind: IpAddrKind) {
    
}
