use std::fs;
use std::path::Path;

pub fn rm(args: &[String]) -> String {
    if args.is_empty() {
        return "rm: missing operand".to_string();
    }

    let mut recursive = false;
    let mut files = Vec::new();

    for arg in args {
        if arg == "-r" {
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
        // ✅ refuse to remove dangerous paths like ".", "..", "/", "./", "../", ".///"
        if is_dangerous_path(file) {
            messages.push(format!("rm: refusing to remove '{}'", file));
            continue;
        }

        let path = Path::new(file);

        if !path.exists() {
            messages.push(format!(
                "rm: cannot remove '{}': No such file or directory",
                file
            ));
            continue;
        }

        if path.is_dir() {
            if recursive {
                match fs::remove_dir_all(path) {
                    Ok(_) => {}
                    Err(e) => {
                        messages.push(format!("rm: cannot remove directory '{}': {}", file, e))
                    }
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

fn is_dangerous_path(path: &str) -> bool {
    let path = Path::new(path);

    // Try to canonicalize the path
    if let Ok(canonical) = path.canonicalize() {
        // Refuse root
        if canonical == Path::new("/") {
            return true;
        }

        // Refuse current dir
        if canonical == std::env::current_dir().unwrap() {
            return true;
        }

        // Optionally, refuse if path escapes current working directory
        let cwd = std::env::current_dir().unwrap();
        if !canonical.starts_with(&cwd) {
            return true;
        }

        // Otherwise, safe
        false
    } else {
        // If canonicalization fails, treat as safe for now (or log warning)
        false
    }
}
