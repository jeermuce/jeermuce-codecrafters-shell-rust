type Args = Vec<String>;
type OutputFile = Option<(PathBuf, bool)>;
pub enum StdType {
    Stdout,
    Stderr,
}

type ParsedInput = Option<(String, Args, OutputFile, StdType)>;
use std::{collections::VecDeque, path::PathBuf};

pub fn parse_input(input: &str) -> ParsedInput {
    let lexer: shlex::Shlex<'_> = shlex::Shlex::new(input);
    let parts: Vec<String> = lexer.collect();
    let mut parts: VecDeque<String> = parts.into_iter().collect();
    if !parts.is_empty() {
        let command: String = parts.pop_front().unwrap();
        let mut args: Args = vec![];
        let mut output_file: OutputFile = None;
        let mut std_type = StdType::Stdout;

        while let Some(arg) = parts.pop_front() {
            match arg.as_str() {
                ">" | ">>" => {
                    if let Some(file) = parts.pop_front() {
                        output_file = Some((PathBuf::from(file), arg == ">>"));
                    } else {
                        eprintln!("If redirecting with > or >>, include output file.");
                        return None;
                    }
                }
                "&>" | ">&" => {
                    if let Some(file) = parts.pop_front() {
                        output_file = Some((PathBuf::from(file), false));
                        std_type = StdType::Stderr;
                    } else {
                        eprintln!("If redirecting with &> or >&, include output file.");
                        return None;
                    }
                }
                "2>" | "2>>" => {
                    if let Some(file) = parts.pop_front() {
                        output_file = Some((PathBuf::from(file), arg == "2>>"));
                        std_type = StdType::Stderr;
                    } else {
                        eprintln!("If redirecting with 2> or 2>>, include output file.");
                        return None;
                    }
                }
                "1>" | "1>>" => {
                    if let Some(file) = parts.pop_front() {
                        output_file = Some((PathBuf::from(file), arg == "1>>"));
                        std_type = StdType::Stdout;
                    } else {
                        eprintln!("If redirecting with 1> or 1>>, include output file.");
                        return None;
                    }
                }
                "2>&1" => {
                    // Redirecting stderr to stdout
                    std_type = StdType::Stderr;
                }
                _ => args.push(arg),
            }
        }
        Some((command, args, output_file, std_type))
    } else {
        None
    }
}
