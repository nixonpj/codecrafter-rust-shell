#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        if input.trim().eq("exit 0") {
            break;
        }
        if input.starts_with("echo ") {
            println!("{}", input[5..].trim());
            continue;
        }
        if input.starts_with("type ") {
            let command = input.trim_start_matches("type ");
            if ["type", "echo", "exit"].contains(&command.trim()) {
                println!("{} is a shell builtin", command.trim());
            } else { 
                println!("{} not found", command.trim());
            }
            continue;
        }
        println!("{}: command not found", input.trim());
    }
}
