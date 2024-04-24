use std::fs;
use node_backend;

fn main() {
    if let Err(error) = make_dir() {
        println!("Failed to create directory: {}", error);
        return;
    }

    println!("Wait Sometimes...");

    if let Err(err) = node_backend::run_command_in_backend() {
        println!("Failed to run command: {}", err);
    }

    println!("Success...");
}

fn make_dir() -> std::io::Result<()> {
    if std::path::Path::new("backend").exists() {
        return Ok(());
    }

    fs::create_dir("backend")?;
    Ok(())
}
