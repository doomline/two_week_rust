// Example of running a struct
// This example takes a struct, runs it in the main function, returning a value


// we want to define the area of rectangles
// first, program in a width and height KVP
struct Rectangle {
    width: u32,
    height: u32,
}
// we want to define a function within the context of rectangle
// we use implementation (impl)
// the area function is moved into the implementaiton
// change the first parameter to self (like python)

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {

    // practice using more methods
    let rect1 = Rectangle (width: 30, height: 50);
    let rect2 = Rectangle (width: 10, height: 40);
    let rect3 = Rectangle (width: 60, height: 45);

    println!("Can rect1 hold rect2?", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?", rect1.can_hold(&rect3));


    println!(
        "The area of the rectangle is {}", 
        rect1.area() // method syntax uses . ( dot ) notation and goes after an instance
    )


    println!("Hello, world!");
}


// we create a function called area 
// it takes the arguments called rectangle, borrowed from the Struct Rectangle
// the value type will be unsigned 32 bit character
// it takes rectangle width ( from rectangle struct ) and rectangle height (from same struct)
// multiplys them together 


/* fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
} */