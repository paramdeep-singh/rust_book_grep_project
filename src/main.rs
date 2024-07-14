use std::env;

fn main() {
    // the convention of including a function's module makes sense for the following line!
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    dbg!(query);
    dbg!(file_path);
}