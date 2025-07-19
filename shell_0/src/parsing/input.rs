use std::io;

use crate::{commands::handle_commands::handle_command, parsing::valide::{command_name, validate_input}};

pub fn reading_input() -> String {
    let mut input = String::new();
    eprint!("$ ");
    if io::stdin().read_line(&mut input).is_err() {
        return "Command '<name>' not found".to_string();
    }
    let trimmed = input.trim();
    
    match validate_input(trimmed) {
       Some(cmd) => handle_command(&cmd, trimmed), 
        None => {
            let cmd = command_name(trimmed);
            format!("Command '{}' not found", cmd)
        }
    }
}
