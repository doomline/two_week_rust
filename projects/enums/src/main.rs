// ENUMS and Pattern matching

//Enums allow you to define a type by enumerating it's possible variants

// Ex. we will be working with IP addresses: there are two types version 4 and version 6
enum IpAddrKind {
    V4,
    V6,
}

// We can now create instances of these variants 
// Both are namespaced under it's the identifier (IpAddrKind)
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;


// we can now define a function that takes either kind
fn route(ip_kind, IpAddrKind) {}

// and call it with any kind
route(IpAddrKind::V4);

// we have the enums but no way of storing actual data that we encounter. 
// we create a struct because the data type we are working with is an IpAddr 
// so the kind of datatype is an Enum, already defined by the name IpAddKind
// the address is the 127.XXX. this will always be a string
struct IpAddr {
    kind: IpAddrKind,
    address: String, 
}

let home = IpAddr {
    kind: IpAddKind::V4,
    address: String::from("127.0.0.1"),
}

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
}

// ENUM writing
// You can also write Enums and inject the datatypes directly
// The following below eliminates the need for a struct
enum IpAddKind{
    V4(String),
    V6(String),
}

// Rust does not have null, it has Option<T>

enum Option<T> {
    Some(T),
    None,
}

let some_number = Some(5);
let some_string = Some("a string");


fn main() {
    println!("Hello, world!");
}
