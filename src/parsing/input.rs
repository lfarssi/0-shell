use std::io;
use crate::{ commands::handle_commands::handle_command, parsing::valide::validate_input };
pub fn reading_input() -> String {
    let mut input = String::new();
    eprint!("$ ");
    let read_inp = io::stdin().read_line(&mut input);

    match read_inp {
        Ok(0) => {
            println!();
            std::process::exit(0);
        }
        Ok(_) => {
            let mut trimmed = input.trim_end().to_string();

            // Keep reading if quotes are not closed
            while
                trimmed
                    .chars()
                    .filter(|&c| c == '"')
                    .count() % 2 != 0 ||
                trimmed
                    .chars()
                    .filter(|&c| c == '\'')
                    .count() % 2 != 0
            {
                eprint!("> ");
                let mut additional_input = String::new();
                if io::stdin().read_line(&mut additional_input).is_err() {
                    return "Error reading input".to_string();
                }
                // keep the newline if inside quotes
                trimmed.push('\n');
                trimmed.push_str(additional_input.trim_end());
            }

            while trimmed.ends_with('\\') {
                // remove the trailing backslash before appending new line
                trimmed.pop();

                eprint!("> ");
                let mut additional_input = String::new();
                if io::stdin().read_line(&mut additional_input).is_err() {
                    return "Error reading input".to_string();
                }
                // append continuation (no extra '\n')
                trimmed.push_str(additional_input.trim_end());
            }

            let tokens = tokenize(&trimmed);

            if tokens.is_empty() {
                return "".to_string();
            }

            let cmd = &tokens[0];
            let args = &tokens[1..];

            match validate_input(cmd) {
                Some(_) => handle_command(cmd, args),
                None => format!("Command '{}' not found", cmd),
            }
        }
        Err(_) => "Command '<name>' not found".to_string(),
    }
}

pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut inside_single = false;
    let mut inside_double = false;

    while let Some(c) = chars.next() {
        match c {
            '"' if !inside_single => {
                inside_double = !inside_double;
            }
            '\'' if !inside_double => {
                inside_single = !inside_single;
            }
            ' ' if !inside_single && !inside_double => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}
