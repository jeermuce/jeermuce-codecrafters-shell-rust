use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::rc::Rc;
use std::str::SplitWhitespace;
use std::{env, io};

use anyhow::Error;

pub fn run_shell(registry: CommandRegistry) {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        if let Some((command, args)) = parse_input(&input) {
            registry.execute(command, args.split_whitespace());
        }
    }
}

pub fn parse_input(input: &str) -> Option<(&str, &str)> {
    let trimmed = input.trim();
    if let Some(pos) = trimmed.find(' ') {
        Some((&trimmed[..pos], &trimmed[pos + 1..]))
    } else if !trimmed.is_empty() {
        Some((trimmed, ""))
    } else {
        None
    }
}

pub type CommandFn = fn(SplitWhitespace, &CommandRegistry);

pub struct CommandRegistry {
    pub commands: HashMap<Rc<str>, CommandFn>,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        CommandRegistry::new()
    }
}

impl CommandRegistry {
    pub fn new() -> Self {
        CommandRegistry {
            commands: HashMap::new(),
        }
    }

    pub fn add_new(&mut self, command: Rc<str>, function: CommandFn) {
        self.commands.insert(command, function);
    }

    pub fn execute(&self, command: &str, args: SplitWhitespace) {
        if let Some(&command_fn) = self.commands.get(command) {
            command_fn(args, self);
        } else if let Some(path) = find_in_path(command) {
            drop(execute_command(args, path))
        } else {
            eprintln!("{command}: command not found");
        }
    }
}

pub fn execute_command(args: SplitWhitespace, program: PathBuf) -> Result<ExitStatus, Error> {
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

pub fn find_in_path(command: &str) -> Option<PathBuf> {
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
