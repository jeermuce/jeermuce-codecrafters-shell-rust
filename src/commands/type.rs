use std::str::SplitWhitespace;

use crate::shell::{find_in_path, CommandRegistry};

pub fn type_command(arr: SplitWhitespace, registry: &CommandRegistry) {
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
