#[allow(unused_imports)]
use std::io::{self, Write};
use std::str::SplitWhitespace;

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let _read = stdin.read_line(&mut input).unwrap();
        let mut arr: SplitWhitespace<'_> = input.split_whitespace();
        match arr.next().unwrap() {
            "exit" => match arr.next().unwrap_or_default() {
                "0" => {
                    std::process::exit(0);
                }
                other => {
                    println!("unknown argument: {}", other);
                }
            },
            "echo" => {
                let string: String = arr.collect::<Vec<&str>>().join(" ");
                println!("{string}");
            }
            _ => {
                println!("{}: command not found", input.trim());
                continue;
            }
        }
    }
}
