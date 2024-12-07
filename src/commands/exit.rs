use crate::shell::CommandRegistry;

pub fn exit_command(arr: Vec<String>, _registry: &CommandRegistry) {
    if let Some(arg) = arr.into_iter().next() {
        match arg.parse::<i32>() {
            Ok(code) => std::process::exit(code),
            Err(_) => eprintln!("exit: numeric argument required"),
        }
    } else {
        std::process::exit(0);
    }
}
