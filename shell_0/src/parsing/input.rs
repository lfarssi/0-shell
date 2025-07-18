use std::io;

use crate::parsing::valide::validate_input;

pub fn reading_input() -> String {
    let mut input = String::new();
    eprint!("$ ");
    if io::stdin().read_line(&mut input).is_err() {
        return "Command '<name>' not found".to_string();
    }
    let trimmed = input.trim();
    
    if validate_input(trimmed.to_string()) {
        // trimmed.to_string()
        "VAlid Command".to_string()
    } else {
        "Command '<name>' not found".to_string()
    }
}
