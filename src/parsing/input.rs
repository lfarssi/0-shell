use crate::{commands::handle_commands::handle_command, parsing::valide::validate_input};
use std::io;

pub fn reading_input() -> Option<String> {
    let mut input = String::new();
    eprint!("$ ");

    match io::stdin().read_line(&mut input) {
        Ok(0) => return None, // Ctrl+D pressed → signal EOF
        Ok(_) => {}
        Err(_) => return None, // handle read error like EOF
    }

    let mut trimmed = input.trim_end().to_string();

    // Keep reading if quotes are not closed
    while !quotes_balanced(&trimmed) {
        eprint!("> ");
        let mut additional_input = String::new();
        match io::stdin().read_line(&mut additional_input) {
            Ok(0) => return None, // Ctrl+D mid-input → exit
            Ok(_) => {}
            Err(_) => return None,
        }
        trimmed.push('\n');
        trimmed.push_str(additional_input.trim_end());
    }

    while trimmed.ends_with('\\') {
        trimmed.pop();
        eprint!("> ");
        let mut additional_input = String::new();
        match io::stdin().read_line(&mut additional_input) {
            Ok(0) => return None,
            Ok(_) => {}
            Err(_) => return None,
        }
        trimmed.push_str(additional_input.trim_end());
    }

    let tokens = tokenize(&trimmed);
    if tokens.is_empty() {
        return Some("".to_string());
    }

    let cmd = &tokens[0];
    let args = &tokens[1..];

    let output = match validate_input(cmd) {
        Some(_) => handle_command(cmd, args),
        None => format!("Command '{}' not found", cmd),
    };

    Some(output)
}

/// Tokenize shell-like: single quotes preserve literally, double quotes allow escapes, spaces split outside quotes
pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut inside_single = false;
    let mut inside_double = false;
    let mut escaped = false;

    while let Some(c) = chars.next() {
        if escaped {
            current.push(c);
            escaped = false;
            continue;
        }

        match c {
            '\\' if !inside_single => escaped = true, // backslash ignored inside single quotes
            '\'' if !inside_double => inside_single = !inside_single,
            '"' if !inside_single => inside_double = !inside_double,
            ' ' | '\t' if !inside_single && !inside_double => {
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

/// Check if quotes are balanced like a shell
fn quotes_balanced(s: &str) -> bool {
    let mut inside_single = false;
    let mut inside_double = false;
    let mut escaped = false;

    for c in s.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        match c {
            '\\' if !inside_single => escaped = true,
            '\'' if !inside_double => inside_single = !inside_single,
            '"' if !inside_single => inside_double = !inside_double,
            _ => {}
        }
    }

    !inside_single && !inside_double
}
