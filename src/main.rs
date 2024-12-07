//use std::cell::RefCell; // TODO: learn how this is used for mutations

use std::rc::Rc;
pub mod shell;
use commands::cd::cd_command;
use commands::echo::echo_command;
use commands::exit::exit_command;
use commands::pwd::pwd_command;
use commands::r#type::type_command;

use shell::{run_shell, CommandRegistry};

pub mod commands;

fn main() {
    let mut registry = CommandRegistry::new();
    registry.add_new(Rc::from("exit"), exit_command);
    registry.add_new(Rc::from("echo"), echo_command);
    registry.add_new(Rc::from("type"), type_command);
    registry.add_new(Rc::from("pwd"), pwd_command);
    registry.add_new(Rc::from("cd"), cd_command);

    run_shell(registry);
}
