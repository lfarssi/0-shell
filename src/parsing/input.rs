use std::io;

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
            println!(); // move to a new line like real shells
            std::process::exit(0);
        }
        Ok(_) => {
            let mut trimmed = input.trim().to_string();

            // Check if input has an unclosed quote
            while trimmed.chars().filter(|&c| c == '"').count() % 2 != 0 {
                eprint!("dquote> ");
                let mut additional_input = String::new();
                if io::stdin().read_line(&mut additional_input).is_err() {
                    return "Error reading input".to_string();
                }
                trimmed.push_str(&additional_input.trim());
            }
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
    let mut current = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                inside_quote = !inside_quote;
            }
            ' ' => {
                if !current.is_empty() {
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
