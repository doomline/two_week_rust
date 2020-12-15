fn main() {
    // variables are always immutable(they can't be changed)
    // if you want to allow a variable to be changed just add "mut" after the let indicator
    let mut x = 5;
    println!("The value x is: {}", x);
    x = 6;
    println!("The value x is: {}", x);

    // constants are always immutable
    // naming convention for them is const <Name = UPPER_CASE>
    // underscores can be added to numeric literals
    const MAX_POINTS: u32 = 100_000;

    /* Shadowing
    shadowing is what takes place when you declare a new variable with the same name as the previous one 
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("the value of x is: {}", x); would print out 12

    */

    // Data Types

    // **** Scalar Types = ints, bools, floats, chars

    // ints can either be signed(i) or unsigned (u) meaning if they need a sign or not (+ or -), unsigned will always be assumed positive by the machine, unsigned can be either 
    let guess: u32 = "42".parse().expect("Not a number");

    // Floats
    //Floats are delineated by the .
    // show what the value is
    let y: f32 = 3.0;

    // Chars
    // Rusts char types are ASCII readable so you're not just limited to english
    let c = 'z';
    let z = 'z';

    // **** Compound Types


    // ** TUPLES
    // these can group multiple values into one type 
    // Rust has two, tuples and arrays

    // Tuples can use comma separated values inside parentheses
    // they don't have to be the same data type

   // let tup: (i32, f64, u8) = (500, 6.4, 1);
    
   let tup = (500, 6.4, 1);

   let (x, y, z) = tup;

   println!("The value fo y is {}", y);

   // you can access a value by using a . Example

   let five_hundred = tup.1;

   // ** ARRAYS
   // arrays are the same in Rust as they are everywhere else
   // they are same datatype, noted by "" around each entry, and encased in brakcets []

   // write the array type as follows 
   // this says [_ data type; _ amount of items]
   // let a: [i32; 5] = [1, 2, 3, 4, 5];

   // you can also initialize arrays the same way
   // let a: [3; 5] it will show a = [3, 3, 3, 3, 3];



   // ********FUNCTIONS

   // functions in Rust work just as they do in other programs
   // call the function in the main function 
   // you can define it outside of the main function 

   another_function();
}

    // ********FUNCTIONS

    // if there are function arguments YOU MUST DECLARE THEM 
    // you can build the function out here
    // functions start with fn, <function_name> (<insert arguments>) {<Insert Code>}

    fn another_function() {
        println!("Another function.");
    }

    // Statements and expressions in function bodies
    // Statements => instructions that perform action and do not return value
    // Expressions evaluate to a resulting value 

    fn example() {
        let y = 6; //statement but the y = 6 is an expression
        5 + 6; //expression 
    }
    