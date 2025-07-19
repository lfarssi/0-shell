use crate::commands::echo::echo;

pub fn handle_command(command: &str, input: &str) -> String {
    match command {
        "echo" => {
            echo(input)
        }
        "pwd" => {
            std::env::current_dir()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|_| "Failed to get current directory".to_string())
        }
        "exit" => {
            std::process::exit(0);
        }
        _ => format!("Handler for '{}' not implemented", command),
    }
}