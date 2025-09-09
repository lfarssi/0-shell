use crate::commands::handle_commands::handle_command;
use crate::parsing::valide::validate_input;
use crate::parsing::input::tokenize;
pub fn echo(input: &[String]) -> String {
    let mut result = Vec::new();

    for arg in input {
        let mut expanded = String::new();
        let mut inside_bt = false;
        let mut cmd = String::new();
        let mut inside_quote = false;
        let mut backtick_depth = 0;

        let mut chars = arg.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '"' if !inside_bt => {
                    inside_quote = !inside_quote;
                    // Do not include quotes in the output
                    continue;
                }
                '`' => {
                    if inside_bt {
                        backtick_depth -= 1;
                        if backtick_depth == 0 {
                            // Run subcommand, which may contain nested backticks
                            let out_str = run_subcommand(&cmd);
                            expanded.push_str(&out_str);
                            cmd.clear();
                            inside_bt = false;
                        } else {
                            cmd.push(c);
                        }
                    } else {
                        inside_bt = true;
                        backtick_depth += 1;
                        cmd.clear();
                    }
                }
                _ => {
                    if inside_bt {
                        cmd.push(c);
                    } else {
                        expanded.push(c);
                    }
                }
            }
        }

        if inside_bt {
            // Handle unclosed backtick by treating it as literal
            expanded.push('`');
            expanded.push_str(&cmd);
        }

        if !expanded.is_empty() {
            result.push(expanded);
        }
    }

    result.join(" ")
}

pub fn run_subcommand(line: &str) -> String {
    let tokens = tokenize(line);
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