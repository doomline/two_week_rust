
use std::env; // Inspection and manipulation of the processes env
use std::fs; // Filesystem manipulation operations

fn main() {
    let args: Vec<String> = env::args().collect(); 
    /* this says store args as a Vector from a string 
    */ 

    let (query, filename) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename) // make a variable "contents" which is made up of reading using the FS module, using the read_file_to_string method with the filename variable as the arguemnt
    .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}