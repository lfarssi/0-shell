use std::io;

use crate::{commands::handle_commands::handle_command, parsing::valide::{command_name, validate_input}};

pub fn reading_input() -> String {
    
    let mut input = String::new();
    eprint!("$ ");

    if io::stdin().read_line(&mut input).is_err() {
        return "Command '<name>' not found".to_string();
    }

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

    if trimmed.len() == 0 {
        return "".to_string();
    } else {
        match validate_input(&trimmed) {
            Some(cmd) => handle_command(&cmd, &trimmed),
            None => {
                let cmd = command_name(&trimmed);
                format!("Command '{}' not found", cmd)
            }
        }
    }
}
