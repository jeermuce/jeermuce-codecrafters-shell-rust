//use std::cell::RefCell; // TODO: learn how this is used for mutations
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::rc::Rc;
use std::str::SplitWhitespace;

use anyhow::Error;

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
        } else if let Some(path) = find_in_path(command) {
            drop(execute_command(args, path))
        } else {
            eprintln!("{command}: command not found");
        }
    }
}
fn main() {
    let mut registry = CommandRegistry::new();
    registry.add_new(Rc::from("exit"), exit_command);
    registry.add_new(Rc::from("echo"), echo_command);
    registry.add_new(Rc::from("pwd"), pwd_command);
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
fn execute_command(args: SplitWhitespace, program: PathBuf) -> Result<ExitStatus, Error> {
    let args: Vec<&str> = args.collect();
    let mut command = Command::new(program);
    command.args(args);

    match command.spawn() {
        Ok(mut child) => match child.wait() {
            Ok(status) => Ok(status),
            Err(e) => Err(anyhow::Error::from(e)),
        },
        Err(e) => Err(anyhow::Error::from(e)),
    }
}

fn pwd_command(_args: SplitWhitespace, _registry: &CommandRegistry) {
    let current_dir = env::current_dir().unwrap();
    println!("{}", current_dir.display());
}

fn type_command(arr: SplitWhitespace, registry: &CommandRegistry) {
    if let Some(command) = arr.into_iter().next() {
        if registry.commands.contains_key(command) {
            println!("{command} is a shell builtin");
        } else if let Some(dir) = find_in_path(command) {
            println!("{command} is {}", dir.display());
        } else {
            eprintln!("{command}: not found");
        }
    }
}

fn find_in_path(command: &str) -> Option<PathBuf> {
    let path = env::var("PATH").unwrap_or_default();
    let mut pathdir: Option<PathBuf> = None;
    for dir in path.split(':') {
        let command_path = Path::new(dir).join(command);
        if command_path.exists() {
            pathdir = Some(command_path);
            break;
        }
    }
    pathdir
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
