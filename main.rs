use std::fs;
use std::io::{self, Write};

fn main() {
    // 1. Read file name
    println!("Please enter your file name:");
    let filename = read_trimmed_line();
    if filename.is_empty() {
        println!("File name cannot be empty!");
        return;
    }

    // 2. Read command
    println!("Please enter your command (backup, restore, delete):");
    let command = read_trimmed_line();

    match command.as_str() {
        "backup" => {
            match backup_file(&filename) {
                Ok(_) => println!("Your backup created: {}.bak", filename),
                Err(e) => println!("Failed to backup: {}", e),
            }
        }
        "restore" => {
            match restore_file(&filename) {
                Ok(_) => println!("Restored backup for file: {}", filename),
                Err(e) => println!("Failed to restore: {}", e),
            }
        }
        "delete" => {
            match delete_file(&filename) {
                Ok(_) => println!("Deleted file: {}", filename),
                Err(e) => println!("Failed to delete: {}", e),
            }
        }
        _ => println!("Invalid command! Please use backup, restore, or delete."),
    }
}

// Helper function to read and trim input
fn read_trimmed_line() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap(); // flush prompt
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

// Backup file by copying it to a .bak file
fn backup_file(filename: &str) -> Result<(), std::io::Error> {
    let backup_name = format!("{}.bak", filename);
    fs::copy(filename, &backup_name)?; // actually use filename
    Ok(())
}

// Restore file by copying from the .bak file
fn restore_file(filename: &str) -> Result<(), std::io::Error> {
    let backup_name = format!("{}.bak", filename);
    fs::copy(&backup_name, filename)?; // actually use filename
    Ok(())
}

// Delete the original file
fn delete_file(filename: &str) -> Result<(), std::io::Error> {
    fs::remove_file(filename)?; // actually use filename
    Ok(())
}
