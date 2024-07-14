use std::env;
use std::fs;

fn main() {
    // Hmm so assigning owner to a the all important args, storing on the heap initially, and 
    // probably just passing slice references around rest of the code
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file!");
    
    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Config {
        Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        }
    }
}
