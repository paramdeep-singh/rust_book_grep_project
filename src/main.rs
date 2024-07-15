use std::env;
use std::process;

use grep_project::Config;

fn main() {
    // Hmm so assigning owner to a the all important args, storing on the heap initially, and 
    // probably just passing slice references around rest of the code
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = grep_project::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
