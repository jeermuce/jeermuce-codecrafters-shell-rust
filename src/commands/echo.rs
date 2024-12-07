use crate::shell::CommandRegistry;

pub fn echo_command(arr: Vec<String>, _registry: &CommandRegistry) {
    println!("{}", arr.join(" "));
}
