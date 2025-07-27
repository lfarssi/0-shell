use std::fs;
use std::path::Path;

pub fn mkdir(input: &str) -> String {
    let args: Vec<&str> = input.split_whitespace().collect();

    if args.len() < 2 {
        return String::from("mkdir: missing operand");
    }

    let mut messages = Vec::new();

    for dir in &args[1..] {
        let path = Path::new(dir);
        match fs::create_dir(path) {
            Ok(_) => {}
            Err(e) => messages.push(format!("mkdir: cannot create directory '{}': {}", dir, e)),
        }
    }

    if messages.is_empty() {
        String::new() // no output on success
    } else {
        messages.join("\n")
    }
}
