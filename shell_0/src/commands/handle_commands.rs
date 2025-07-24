use crate::commands::{echo::echo, exit::exit, pwd::pwd};

pub fn handle_command(command: &str, input: &str) -> String {
    match command {
        "echo" | r#""echo""# => {
            echo(input)
        }
        "pwd" => {
           pwd()
        }
        "exit" => {
           exit()
        }
        _ => format!("Handler for '{}' not implemented", command),
    }
}