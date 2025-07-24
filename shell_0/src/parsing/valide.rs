pub fn validate_input(input: &str) -> Option<String> {
    let command = command_name(input);
    let valid = match command {
        "echo" | r#""echo""# | "cd" | "pwd" | "cat" | "cp" | "mv" | "mkdir" | "exit" => true,
        "ls" => {
            let mut  args = input.split_whitespace().skip(1);
            args.all(|arg| arg == "-l" || arg == "-a" || arg == "-F")
        }
        "rm" => {
            let mut args = input.split_whitespace().skip(1);
            args.all(|arg| arg == "-r")
        }
        _ => false,
    };

    if valid {
        Some(command.to_string())
    } else {
        None
    }
}


pub fn command_name(s: &str) -> &str {

     s.split_whitespace().next().unwrap_or("")
}