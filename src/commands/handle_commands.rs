use crate::commands::{echo::echo, exit::exit, pwd::pwd,mkdir::mkdir,clear::clear};

pub fn handle_command(command: &str, input: &str) -> String {
    match command {
        "echo" | r#""echo""# =>  echo(input),
        "pwd" => pwd(),
        "exit" => exit(),
         "mkdir" => mkdir(input),
          "clear" => clear(),
        _ => format!("Handler for '{}' not implemented", command),
    }
}