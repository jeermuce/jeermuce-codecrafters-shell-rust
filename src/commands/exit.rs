use std::str::SplitWhitespace;

use crate::shell::CommandRegistry;

pub fn exit_command(arr: SplitWhitespace, _registry: &CommandRegistry) {
    match arr.into_iter().next() {
        Some("0") | Some("") | Some(" ") => std::process::exit(0),
        Some(other) => eprintln!("unknown argument: {other}"),
        None => std::process::exit(0),
    }
}
