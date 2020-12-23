
use std::error::Error;
use std::env; // Inspection and manipulation of the processes env
use std::fs; // Filesystem manipulation operations

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

    run(config);

    let contents = fs::read_to_string(config.filename) // make a variable "contents" which is made up of reading using the FS module, using the read_file_to_string method with the filename variable as the argument
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_file_to_string(config.filename)?;

        println!("With text:\n", contents);

        Ok(())
}


// Create a struct called config with params query and filename made of type String

struct Config {
    query: String,
    filename: String,
}

// implement the Config struct every time the "new" function is ran. 
// it will take the arguments from the CLI and returned them as owned values

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
}
}
