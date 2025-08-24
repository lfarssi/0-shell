use std::fs;
use std::path::Path;

pub fn mkdir(args: &[String]) -> String {
    if args.is_empty() {
        return String::from("mkdir: missing operand");
    }

    let mut messages = Vec::new();

    for dir in args {
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
