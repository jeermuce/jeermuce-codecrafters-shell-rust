use std::env;

use crate::shell::CommandRegistry;

pub fn pwd_command(_args: Vec<String>, _registry: &CommandRegistry) {
    let current_dir = env::current_dir().unwrap();
    println!("{}", current_dir.display());
}
