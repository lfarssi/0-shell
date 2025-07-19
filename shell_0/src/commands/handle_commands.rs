use crate::commands::{echo::echo, pwd::pwd};

pub fn handle_command(command: &str, input: &str) -> String {
    match command {
        "echo" => {
            echo(input)
        }
        "pwd" => {
           pwd()
        }
        "exit" => {
            std::process::exit(0);
        }
        _ => format!("Handler for '{}' not implemented", command),
    }
}