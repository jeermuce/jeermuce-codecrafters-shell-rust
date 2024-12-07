use std::collections::HashMap;
use std::io::{self, Write};
use std::str::SplitWhitespace;

type CommandFn = fn(SplitWhitespace, &CommandRegistry);

struct CommandRegistry {
    commands: HashMap<&'static str, CommandFn>,
}

impl CommandRegistry {
    fn new() -> Self {
        let commands: HashMap<&'static str, CommandFn> = HashMap::new();
        CommandRegistry { commands }
    }

    fn add_new(&mut self, command: &'static str, function: CommandFn) {
        self.commands.insert(command, function);
    }

    fn execute(&self, command: &str, args: SplitWhitespace) {
        if let Some(&command_fn) = self.commands.get(command) {
            command_fn(args, self);
        } else {
            eprintln!("{}: command not found", command);
        }
    }
}
fn main() {
    let mut registry = CommandRegistry::new();
    registry.add_new("exit", exit_command);
    registry.add_new("echo", echo_command);
    registry.add_new("type", type_command);
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let _ = stdin.read_line(&mut input).unwrap();
        let mut arr = input.split_whitespace();

        if let Some(command) = arr.next() {
            registry.execute(command, arr);
        }
    }
}

fn type_command(arr: SplitWhitespace, registry: &CommandRegistry) {
    if let Some(command) = arr.peekable().peek() {
        if registry.commands.contains_key(*command) {
            println!("{} is a shell builtin", command);
        } else {
            eprintln!("{}: not found", command);
        }
    }
}

fn exit_command(arr: SplitWhitespace, _registry: &CommandRegistry) {
    if let Some(arg) = arr.peekable().peek() {
        match *arg {
            "0" => std::process::exit(0),
            other => eprintln!("unknown argument: {other}"),
        }
    }
}

fn echo_command(arr: SplitWhitespace, _registry: &CommandRegistry) {
    let output: String = arr.collect::<Vec<&str>>().join(" ");
    println!("{}", output);
}
