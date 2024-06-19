// Import the standard library's I/O module
use std::env;

fn main() {
    // Collect all command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Check if a name was provided
    if args.len() > 1 {
        // Print a personalized greeting
        println!("Hello, {}!", args[1]);
    } else {
        // Print a generic greeting
        println!("Hello, World!");
    }
}