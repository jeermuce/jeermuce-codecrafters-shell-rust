#!/usr/bin/env run-cargo-script
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let path = Path::new("src");
    println!("Starting update on directory: {:?}", path);
    update_tree(&path);
    println!("Update complete.");
}

fn update_tree(path: &Path) {
    if path.is_dir() {
        println!("Handling directory: {:?}", path);
        handle_directory(path);

        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            if entry_path.is_dir() {
                update_tree(&entry_path);
            }
        }
    }
}

fn handle_directory(dir_path: &Path) {
    let parent_dir = dir_path.parent().unwrap();
    let dir_name = dir_path.file_name().unwrap().to_str().unwrap();
    let mod_file_name = format!("{}.rs", dir_name);
    let mod_file_path = parent_dir.join(&mod_file_name);

    println!("Creating/updating mod file: {:?}", mod_file_path);

    let mut mod_statements = vec![];

    if mod_file_path.exists() {
        let file = File::open(&mod_file_path).unwrap();
        let reader = BufReader::new(file);
        println!("Reading existing mod statements from: {:?}", mod_file_path);
        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("pub mod") {
                mod_statements.push(line);
            }
        }
    }

    for entry in fs::read_dir(dir_path).unwrap() {
        let entry = entry.unwrap();
        let entry_name = entry.file_name();
        let entry_name_str = entry_name.to_str().unwrap();
        if entry.path().is_file() {
            let mod_statement = format!("pub mod {};", entry_name_str.trim_end_matches(".rs"));
            if !mod_statements.contains(&mod_statement) {
                mod_statements.push(mod_statement.clone());
                println!("Adding mod statement: {}", mod_statement);
            }
        }
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&mod_file_path)
        .unwrap();
    println!("Writing mod statements to: {:?}", mod_file_path);
    for statement in mod_statements {
        writeln!(file, "{}", statement).unwrap();
    }
}
