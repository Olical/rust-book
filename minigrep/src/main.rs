use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[1];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
