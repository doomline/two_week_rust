// call the external crate, use this so people know where you got the resource from
extern crate rand;


// you wont know which traits to use from a crate
// You'll need to consult the documentation by doing cargo doc --open
// use library std::io is same as python import
use std::io;
use std::cmp::Ordering;
// rand::Rng gives us the trait range, think FROM <trait> IMPORT
use rand::Rng;



fn main() {
    println!("Guess the number!");
    // gives the random number generator, one that is local to current thread of execution
    // gen_range() then generates the number. need it from 1-100 so it goes start at 1, stop at 101, generating 1-100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // loop creates infinite loops
    loop{
        println!("Please input your guess.");

        // creating new variable, remmeber, variables in Rust are not mutable (can be changed)
        // you have to explicitly state that the variable will be mutable
        // let mut guess creates a mutable variable bound to an empty instance of a string
        
        let mut guess = String::new(); // "::" notes an associated function
        // "&"indicates reference
        io::stdin().read_line(&mut guess) // uses the stdin instance, calls the readline method to get user input, passes argument &mut guess
            .expect("Failed to read line");
        // this is a new variable called guess. This is called shadowing, you can use it to convert 
        // variable types instead of having to create 2 unique variables such as guess_str() and guess
        // the : after guess tells Rust the type, 
        // .trim() removes whitespace
        // .parse() prases the string to a number
        // . we use .expect() because .expect signals we are expecting an error, if no error then print the value
        // changing to match instead of expect lets us deal with issues and bugs on the fly
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

       

        // {} is referred to as a primal place holder
        // place holders hold values 
        println!("You guessed: {}", guess);


        // we use a match expression which is made up of arms.
        // an arm consists of a pattern and code that should be run if the value given to the beginning of the match expression fits that arm's pattern
        // match is just like a function except the ARM is already established
        // the pattern is something to compare it to (argument)
        match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        } 
        }
    }   

}
