//use std::cell::RefCell; // TODO: learn how this is used for mutations
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;
use std::str::SplitWhitespace;

type CommandFn = fn(SplitWhitespace, &CommandRegistry);

struct CommandRegistry {
    commands: HashMap<Rc<str>, CommandFn>,
}

impl CommandRegistry {
    fn new() -> Self {
        CommandRegistry {
            commands: HashMap::new(),
        }
    }

    fn add_new(&mut self, command: Rc<str>, function: CommandFn) {
        self.commands.insert(command, function);
    }

    fn execute(&self, command: &str, args: SplitWhitespace) {
        if let Some(&command_fn) = self.commands.get(command) {
            command_fn(args, self);
        } else {
            eprintln!("{command}: command not found");
        }
    }
}
fn main() {
    let mut registry = CommandRegistry::new();
    registry.add_new(Rc::from("exit"), exit_command);
    registry.add_new(Rc::from("echo"), echo_command);
    registry.add_new(Rc::from("teco"), |args, _| {
        println!("{}", args.collect::<Vec<&str>>().join(" "))
    });

    registry.add_new(Rc::from("type"), type_command);
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut arr = input.split_whitespace();

        if let Some(command) = arr.next() {
            registry.execute(command, arr);
        }
    }
}

fn type_command(arr: SplitWhitespace, registry: &CommandRegistry) {
    if let Some(command) = arr.into_iter().next() {
        if registry.commands.contains_key(command) {
            println!("{command} is a shell builtin");
        } else {
            check_path(command);
        }
    }
}

fn check_path(command: &str) {
    let path = env::var("PATH").unwrap_or_default();
    let mut found = false;
    for dir in path.split(':') {
        let command_path = Path::new(dir).join(command);
        if command_path.exists() {
            println!("{command} is {}", command_path.display());
            found = true;
            break;
        }
    }
    if !found {
        eprintln!("{command}: not found");
    }
}

fn exit_command(arr: SplitWhitespace, _registry: &CommandRegistry) {
    match arr.into_iter().next() {
        Some("0") | Some("") | Some(" ") => std::process::exit(0),
        Some(other) => eprintln!("unknown argument: {other}"),
        None => std::process::exit(0),
    }
}
fn echo_command(arr: SplitWhitespace, _registry: &CommandRegistry) {
    println!("{}", arr.collect::<Vec<&str>>().join(" "));
}
