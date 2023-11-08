use std::process::{Command, exit};

fn perform_backup() {

    let mkdir_command = Command::new("mkdir")
        .arg("-p")
        .arg("backup_directory")
        .output()
        .expect("Failed to execute mkdir command");

    if !mkdir_command.status.success() {
        eprintln!("Error: Failed to create backup directory.");
        exit(1);
    }

    let cp_command = Command::new("cp")
        .arg("-r")
        .arg("important_data/")
        .arg("backup_directory/")
        .output()
        .expect("Failed to execute copy command");

    if !cp_command.status.success() {
        eprintln!("Error: Failed to copy files to backup directory.");
        exit(1);
    }

    println!("Successfully backed up files!");
}

fn main() {
    perform_backup();
}
