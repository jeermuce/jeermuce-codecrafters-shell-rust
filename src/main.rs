//use std::cell::RefCell; // TODO: learn how this is used for mutations

pub mod parser;
pub mod shell;

use commands::cd::cd_command;
use std::rc::Rc;

use commands::echo::echo_command;
use commands::exit::exit_command;
use commands::pwd::pwd_command;
use commands::r#type::type_command;

use shell::{run_shell, CommandRegistry};

pub mod commands;

fn main() {
    let mut registry = CommandRegistry::new();
    registry.add_new(Rc::from("exit".to_string()), exit_command);
    registry.add_new(Rc::from("echo".to_string()), echo_command);
    registry.add_new(Rc::from("type".to_string()), type_command);
    registry.add_new(Rc::from("pwd".to_string()), pwd_command);
    registry.add_new(Rc::from("cd".to_string()), cd_command);

    run_shell(registry);
}
