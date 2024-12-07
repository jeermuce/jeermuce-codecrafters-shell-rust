use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::rc::Rc;
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
            let args = shellwords::split(&args).unwrap_or_else(|_| vec![]);
            registry.execute(command, args);
        }
    }
}

fn parse_input(input: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();

    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        Some((parts[0].to_string(), "".to_string()))
    }
}

pub type CommandFn = fn(Vec<String>, &CommandRegistry);

pub struct CommandRegistry {
    pub commands: HashMap<Rc<String>, CommandFn>,
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

    pub fn add_new(&mut self, command: Rc<String>, function: CommandFn) {
        self.commands.insert(command, function);
    }

    pub fn execute(&self, command: String, args: Vec<String>) {
        if let Some(builtin) = self.commands.get(&Rc::from(command.clone())) {
            builtin(args, self);
        } else if let Some(program) = find_in_path(command.clone()) {
            match execute_command(args, program) {
                Ok(status) => {
                    if !status.success() {
                        eprintln!("{}: failed with status {}", command, status);
                    }
                }
                Err(e) => eprintln!("{}: failed with error {:?}", command, e),
            }
        } else {
            eprintln!("{command}: not found");
        }
    }
}

pub fn execute_command(args: Vec<String>, program: PathBuf) -> Result<ExitStatus, Error> {
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

pub fn find_in_path(command: String) -> Option<PathBuf> {
    let path = env::var("PATH").unwrap_or_default();
    let mut pathdir: Option<PathBuf> = None;
    for dir in path.split(':') {
        let command_path = Path::new(dir).join(&command);
        if command_path.exists() {
            pathdir = Some(command_path);
            break;
        }
    }
    pathdir
}
