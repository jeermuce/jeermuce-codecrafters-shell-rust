use crate::shell::{find_in_path, CommandRegistry};

pub fn type_command(arr: Vec<String>, registry: &CommandRegistry) {
    if let Some(command) = arr.into_iter().next() {
        if registry.commands.contains_key(&command) {
            // expected reference `&_` got `std::String::String`
            println!("{command} is a shell builtin");
        } else if let Some(dir) = find_in_path(command.clone()) {
            //
            println!("{command} is {}", dir.display());
        } else {
            eprintln!("{command}: not found");
        }
    }
}
