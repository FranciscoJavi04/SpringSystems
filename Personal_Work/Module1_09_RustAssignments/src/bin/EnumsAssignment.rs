use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),            // Directory path
    Display(String),         // File path
    Create(String, String),  // File path and content
    Remove(String),          // File path
    Pwd,                     // Print working directory
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            let status = Command::new("ls")
                .arg(path)
                .status();

            match status {
                Ok(s) if s.success() => {}
                Ok(_) => eprintln!("Failed to list files."),
                Err(_) => eprintln!("Failed to execute ls."),
            }
        }

        FileOperation::Display(path) => {
            let status = Command::new("cat")
                .arg(path)
                .status();

            match status {
                Ok(s) if s.success() => {}
                Ok(_) => eprintln!("Failed to display file contents."),
                Err(_) => eprintln!("Failed to execute cat."),
            }
        }

        FileOperation::Create(path, content) => {
            let command = format!("echo '{}' > {}", content, path);

            let status = Command::new("sh")
                .arg("-c")
                .arg(command)
                .status();

            match status {
                Ok(s) if s.success() => println!("File '{}' created successfully.", path),
                Ok(_) => eprintln!("Failed to create file."),
                Err(_) => eprintln!("Failed to execute create command."),
            }
        }

        FileOperation::Remove(path) => {
            let status = Command::new("rm")
                .arg(path.clone())
                .status();

            match status {
                Ok(s) if s.success() => println!("File '{}' removed successfully.", path),
                Ok(_) => eprintln!("Failed to remove file."),
                Err(_) => eprintln!("Failed to execute rm."),
            }
        }

        FileOperation::Pwd => {
            let status = Command::new("pwd").status();

            match status {
                Ok(s) if s.success() => {}
                Ok(_) => eprintln!("Failed to print working directory."),
                Err(_) => eprintln!("Failed to execute pwd."),
            }
        }
    }
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        let choice = read_input("Enter your choice (0-5): ");

        let operation = match choice.as_str() {
            "1" => {
                let dir_path = read_input("Enter directory path: ");
                Some(FileOperation::List(dir_path))
            }
            "2" => {
                let file_path = read_input("Enter file path: ");
                Some(FileOperation::Display(file_path))
            }
            "3" => {
                let file_path = read_input("Enter file path: ");
                let content = read_input("Enter content: ");
                Some(FileOperation::Create(file_path, content))
            }
            "4" => {
                let file_path = read_input("Enter file path: ");
                Some(FileOperation::Remove(file_path))
            }
            "5" => Some(FileOperation::Pwd),
            "0" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Please enter a number from 0 to 5.");
                None
            }
        };

        if let Some(op) = operation {
            perform_operation(op);
        }
    }
}