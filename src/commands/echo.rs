use std::str::SplitWhitespace;

use crate::shell::CommandRegistry;

pub fn echo_command(arr: SplitWhitespace, _registry: &CommandRegistry) {
    println!("{}", arr.collect::<Vec<&str>>().join(" "));
}
