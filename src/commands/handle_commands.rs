use crate::commands::{ls::ls, rm::rm, mv::mv, cd::cd, cat::cat, echo::echo, exit::exit, cp::cp, pwd::pwd, mkdir::mkdir, clear::clear};

pub fn handle_command(command: &str, input: &[String]) -> String {
    match command {
        "echo" | r#""echo""# =>  echo(input),
        "pwd" => pwd(),
        "exit" => exit(),
        "mkdir" => mkdir(input),
        "clear" => clear(),
        "cp" => cp(input),
        "rm" => rm(input),
        "mv" => mv(input),
        "cat" => cat(input),
        "ls" => ls(input),
        "cd" => cd(input),
        _ => format!("Command '{}' not found", command),
    }
}