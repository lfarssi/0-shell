use crate::commands::{echo::echo, exit::exit, pwd::pwd,mkdir::mkdir};

pub fn handle_command(command: &str, input: &str) -> String {
    match command {
        "echo" | r#""echo""# =>  echo(input),
        "pwd" => pwd(),
        "exit" => exit(),
         "mkdir" => mkdir(input),
        _ => format!("Handler for '{}' not implemented", command),
    }
}