use std::io;
use crate::commands::echo::run_subcommand;
use crate::{
    commands::handle_commands::handle_command,
    parsing::valide::validate_input,
};

pub fn reading_input() -> String {
    let mut input = String::new();
    eprint!("$ ");
    let read_inp = io::stdin().read_line(&mut input);
    match read_inp {
        Ok(0) => {
           
            std::process::exit(0);
        }
        Ok(_) => {
            let trimmed = input.trim().to_string();
            let tokens = tokenize(&trimmed);

            if tokens.is_empty() {
                return "".to_string();
            }

            let cmd = &tokens[0]; // command name
            let args = &tokens[1..]; // arguments

            match validate_input(cmd) {
                Some(_) => handle_command(cmd, args),
                None => format!("Command '{}' not found", cmd),
            }
        }
        Err(_) => {
            return "Command '<name>' not found".to_string();
        }
    }
}

pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut inside_quote = false;
    let mut inside_backtick = false;
    let mut current = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if !inside_backtick {
                    inside_quote = !inside_quote;
                    current.push(c);
                } else {
                    current.push(c);
                }
            }
            '`' => {
                if !inside_quote {
                    if inside_backtick {
                        // End of backtick, execute subcommand
                        let output = run_subcommand(&current);
                        tokens.push(output);
                        current.clear();
                        inside_backtick = false;
                    } else {
                        inside_backtick = true;
                        current.clear(); // Start collecting subcommand
                    }
                } else {
                    current.push(c);
                }
            }
            ' ' => {
                if inside_quote || inside_backtick {
                    current.push(c);
                } else if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}