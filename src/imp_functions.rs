use std::process::{Command, Output};
use std::io;
use std::fs;

pub fn execute_npm_command(cmd1: &str , cmd2: &str) -> io::Result<Output> {
    Command::new("npm")
        .arg(cmd1)
        .arg(cmd2)
        .output()
}

pub fn create_folder(folder_name: &str) -> io::Result<()> {
    fs::create_dir_all(folder_name).map_err(|e| {
        eprintln!("Error creating folder '{}': {}", folder_name, e);
        io::Error::new(io::ErrorKind::Other, format!("Failed to create folder '{}'", folder_name))
    })
}
