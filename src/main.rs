#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let _read = stdin.read_line(&mut input).unwrap();

        match input.trim() {
            "exit 0" => {
                std::process::exit(0);
            }
            _ => {
                println!("{}: command not found", input.trim());
                continue;
            }
        }
    }
}
