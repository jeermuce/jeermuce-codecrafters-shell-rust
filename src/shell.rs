use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::rc::Rc;
use std::{env, io};

use crate::parser::{parse_input, StdType};

pub fn run_shell(registry: CommandRegistry) {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        if let Some((command, args, output_file, std_type)) = parse_input(&input) {
            registry.execute(command, args, output_file, std_type);
        }
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

    pub fn execute(
        &self,
        command: String,
        args: Vec<String>,
        output_file: Option<(PathBuf, bool)>,
        std_type: Option<StdType>,
    ) {
        if let Some(builtin) = self.commands.get(&Rc::from(command.clone())) {
            builtin(args, self);
        } else if let Some(program) = find_in_path(command.clone()) {
            match execute_command(args, program, output_file, std_type) {
                Ok(status) => {
                    if !status.success() {
                        eprintln!("{}: failed with status {}", command, status);
                    }
                }
                Err(e) => eprintln!("{}: failed with error {:?}", command, e),
            }
        } else {
            //command not found
            eprintln!("{command}: command not found");
        }
    }
}

pub fn execute_command(
    args: Vec<String>,
    program: PathBuf,
    output_file: Option<(PathBuf, bool)>,
    std_type: Option<StdType>,
) -> Result<ExitStatus, io::Error> {
    let mut command = Command::new(program);
    command.args(args);

    let output = command.output()?;

    match (output_file, std_type) {
        (Some((file_path, append)), Some(StdType::Stdout)) | (Some((file_path, append)), None) => {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(append)
                .open(file_path)?;
            file.write_all(&output.stdout)?;
        }
        (Some((file_path, append)), Some(StdType::Stderr)) => {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(append)
                .open(file_path)?;
            file.write_all(&output.stderr)?;
        }
        (None, Some(StdType::Stdout)) | (None, None) => {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        (None, Some(StdType::Stderr)) => {
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }

    Ok(ExitStatus::from_raw(output.status.into_raw()))
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
