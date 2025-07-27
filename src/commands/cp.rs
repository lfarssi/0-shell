
use std::path::Path;

pub fn cp(input: &str) -> String {
    let args: Vec<&str> = input.trim().split_whitespace().collect();
    if args.len() < 2 {
        return "cp: missing file operand".to_string();
    }

    let sources = &args[..args.len() - 1];
    let destination = args[args.len() - 1];

    let dest_path = Path::new(destination);

    if sources.len() > 1 && (!dest_path.exists() || !dest_path.is_dir()) {
        return format!("cp: target '{}' is not a directory", destination);
    }

    String::new() 
}
