use std::{env, str::SplitWhitespace};

use crate::shell::CommandRegistry;

pub fn pwd_command(_args: SplitWhitespace, _registry: &CommandRegistry) {
    let current_dir = env::current_dir().unwrap();
    println!("{}", current_dir.display());
}
