# Rust Backup Bot

`rust_backup_bot` is a simple backup automation tool written in Rust. It's designed to quickly and safely back up important data by creating a backup directory and copying specified files or directories to that location.

## Features

- **Simple and Fast**: Leverages the speed of Rust for efficient file handling.
- **Easy to Use**: Run the tool with a single command.
- **Cross-Platform**: Works on any system where Rust can be compiled, with specific support for Unix-like systems.

## Getting Started

### Prerequisites

- Rust and Cargo (the Rust package manager) installed on your machine. You can install them from the [official Rust website](https://www.rust-lang.org/tools/install).

### Installation

Clone the repository to your local machine:

```sh
git clone https://github.com/yourusername/rust_backup_bot.git
cd rust_backup_bot
## Running the Backup Tool

After you have built the project, you can run the backup tool using the following steps.

### Step 1: Configuration

Before running the tool, ensure that the `perform_backup` function in `main.rs` has the correct paths set for your source data and backup directory:

```rust
fn perform_backup() {
    // Replace 'source_data/' with the path to the data you want to back up.
    // Replace 'backup_directory/' with the path to where you want your backup stored.
    let src_dir = "source_data/";
    let dest_dir = "backup_directory/";
    // ... rest of the function ...
}

```
### Running

- cargo build --release
  
Compile the project with Cargo to create an executable:
This will compile the source code and place the executable in the target/release directory.

### Run the Backup Tool:

-./target/release/rust_backup_bot

 Or you can run it using Cargo:

 cargo run --release
 Verify the Backup:
 Check the contents of your backup directory to ensure that the backup was 
 successful.


