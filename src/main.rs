use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::env;

// Only for Windows
fn main() {
    println!("Welcome to Leaf Shell\nType 'help' to see a list of commands");

    let mut history: Vec<String> = Vec::new();
    loop {
        // Print prompt
        print!("~> ");
        io::stdout().flush().unwrap();

        // Read input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line");
            continue;
        }

        let mut input = input.trim(); // was not mut
        if input.is_empty() {
            continue;
        }

        if input == "history" {
            for (i, cmd) in history.iter().enumerate() {
                println!("{}: {}", i + 1, cmd);
            }
            continue;
        } else {
            history.push(input.to_string());
        }

        if input == "exit" {
            break;
        }

        if input == "ls" {
            input = "dir";
        }

        if input == "cls" {
            input = "clear";
        }

        if input == "out" {
            input = "cd ..";
        }

        if input == "help" {
            println!(
                "Leaf Shell Commands:
        - cd [dir]     Change directory
        - tree         Something fun
        - clear        Clear screen
        - history      Show command history
        - exit         Quit shell
        - help         Show this message
        Other input is passed to Windows cmd.
        "
            );
            continue;
        }


        if input == "tree" {
            println!(r#"
                        /\
        __   _     __  /**\    __       __
   |   |_   /_\  |__  /* **\  |__  |_| |_  |   |
   |__ |__ /   \ |   /*** **\  __| | | |__ |__ |__
                    /  **  **\
                   / **  ** **\
                  /****    ****\
                 /***  **  ** **\
                       ||||
                       ||||
                    "#);
            continue;
        }

        // Split command and arguments
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        if command == "cd" {
            if let Some(path) = args.get(0) {
                if let Err(e) = env::set_current_dir(path) {
                    eprintln!("cd error: {}", e);
                }
            } else {
                eprintln!("cd: missing argument");
            }
            continue;
        }

        let status = Command::new("sh")
            .arg("-c") 
            .arg(input)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();


        match status {
            Ok(s) => {
                if !s.success() {
                    eprintln!("Process exited with status: {}", s);
                }
            }
            Err(e) => eprintln!("Failed to execute '{}': {}", command, e),
        }
    }

    println!("Thanks for using Leaf Shell");
}