use std::fs;
use std::path::Path;

pub fn rm(args: &[String]) -> String {
    if args.is_empty() {
        return "rm: missing operand".to_string();
    }

    let mut recursive = false;
    let mut files = Vec::new();

    for arg in args {
        if arg == "-r" || arg == "-R" {
            recursive = true;
        } else {
            files.push(arg);
        }
    }

    if files.is_empty() {
        return "rm: missing operand".to_string();
    }

    let mut messages = Vec::new();

    for file in files {
        let path = Path::new(file);
        if path.is_dir() {
            if recursive {
                match fs::remove_dir_all(path) {
                    Ok(_) => {}
                    Err(e) => messages.push(format!("rm: cannot remove directory '{}': {}", file, e)),
                }
            } else {
                messages.push(format!("rm: cannot remove '{}': Is a directory", file));
            }
        } else {
            match fs::remove_file(path) {
                Ok(_) => {}
                Err(e) => messages.push(format!("rm: cannot remove '{}': {}", file, e)),
            }
        }
    }

    if messages.is_empty() {
        String::new()
    } else {
        messages.join("\n")
    }
}
