use anyhow::Error;
use shlex;
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::rc::Rc;
use std::{env, io};

pub fn run_shell(registry: CommandRegistry) {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        if let Some((command, args)) = parse_input(&input) {
            registry.execute(command, args);
        }
    }
}

fn parse_input(input: &str) -> Option<(String, Vec<String>)> {
    let lexer = shlex::Shlex::new(input);
    let parts: Vec<String> = lexer.collect();
    if !parts.is_empty() {
        let command = parts[0].clone();
        let args = parts[1..].to_vec();
        Some((command, args))
    } else {
        None
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
            //command not found
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
