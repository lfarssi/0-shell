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


        // Prevent removing special directories
        if file == "." || file == ".." || file == "/" || file == "./" || file == "../" ||file == "~" {
            messages.push(format!("rm: refusing to remove '{}'", file));
            continue;
        }

        let path = Path::new(file);

        // Check if current working directory is inside or is the directory to be deleted
        if path.is_dir() {
            if let Ok(current_dir) = std::env::current_dir() {
                if let (Ok(target), Ok(cwd)) = (path.canonicalize(), current_dir.canonicalize()) {
                    if cwd == target || cwd.starts_with(&target) {
                        messages.push(format!(
                            "rm: refusing to remove '{}': Directory is current working directory or its ancestor",
                            file
                        ));
                        continue;
                    }
                }
            }
        }


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
