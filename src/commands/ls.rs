use std::fs;
pub fn ls(args: &[String]) -> String {
     let mut output = String::new();

    // Determine the directory to list
    let dir_path = if args.is_empty() {
        "." // current directory if no argument
    } else {
        &args[0] // first argument = directory
    };

    match fs::read_dir(dir_path) {
        Ok(entries) => {
            let mut items = Vec::new();

            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let name = entry.file_name().to_string_lossy().to_string();
                        items.push(name);
                    }
                    Err(e) => {
                        output.push_str(&format!("ls: error reading entry: {}\n", e));
                    }
                }
            }

            items.sort();

            for item in items {
                output.push_str(&format!("{}\n", item));
            }
        }
        Err(e) => {
            output.push_str(&format!("ls: {}: {}\n", dir_path, e));
        }
    }

    output
}