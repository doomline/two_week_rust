
use std::env; // Inspection and manipulation of the processes env
use std::process;

use rust_cli::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); 
    /* this says store args as a Vector from a string 
    */ 

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = rust_cli::run(config) {
        println!("Application error: {}", e)
    }

    process::exit(1)

    let contents = fs::read_to_string(config.filename) // make a variable "contents" which is made up of reading using the FS module, using the read_file_to_string method with the filename variable as the argument

    println!("With text:\n{}", contents);
}

