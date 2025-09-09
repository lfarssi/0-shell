use crate::commands::handle_commands::handle_command;
use crate::parsing::valide::validate_input;
use crate::parsing::input::tokenize;
pub fn echo(input: &[String]) -> String {
    let mut result = Vec::new();

    for arg in input {
        let mut expanded = String::new();
        let mut inside_bt = false;
        let mut cmd = String::new();

        for c in arg.chars() {
            if c == '`' && !inside_bt {
                inside_bt = true;
                cmd.clear();
            } else if c == '`' && inside_bt {
                // Run subcommand
                let out_str = run_subcommand(&cmd);
                expanded.push_str(&out_str);
                cmd.clear();
                inside_bt = false;
            } else if inside_bt {
                cmd.push(c);
            } else {
                expanded.push(c);
            }
        }

        if inside_bt {
            // Handle unclosed backtick
            expanded.push('`');
            expanded.push_str(&cmd);
        }

        result.push(expanded);
    }

    result.join(" ")
}

pub fn run_subcommand(line: &str) -> String {
    let tokens: Vec<String> = tokenize(line); // Use tokenize to handle nested commands
    if tokens.is_empty() {
        return "".to_string();
    }
    let cmd = &tokens[0];
    let args = &tokens[1..];
    match validate_input(cmd) {
        Some(_) => handle_command(cmd, args),
        None => {
            match std::process::Command::new(cmd).args(args).output() {
                Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
                Err(_) => format!("Command '{}' not found", cmd),
            }
        }
    }
}