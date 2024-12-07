use std::{env, path::Path};

use crate::shell::CommandRegistry;

pub fn cd_command(args: Vec<String>, _registry: &CommandRegistry) {
    if let Some(dir) = args.into_iter().next() {
        match dir.chars().next() {
            Some('~') => {
                if let Some(home_dir) = dirs::home_dir() {
                    let resolved = home_dir.join(&dir[1..]);
                    if env::set_current_dir(&resolved).is_err() {
                        eprintln!("{}: No such file or directory", resolved.display());
                    }
                } else {
                    eprintln!("Unable to determine home directory.");
                }
            }
            None => println!(),
            _ => {
                let other = env::set_current_dir(Path::new(&dir));
                if other.is_err() {
                    eprintln!("{}: No such file or directory", dir);
                }
            }
        }
    } else {
        println!("tis");
    }
}
