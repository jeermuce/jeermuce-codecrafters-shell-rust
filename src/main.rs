#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    let _read = stdin.read_line(&mut input).unwrap();

    match input.trim() {
        "exit" => {
            println!("done");
            std::process::exit(0);
        }
        _ => {
            println!("{}: command not found", input.trim());
        }
    }
}
