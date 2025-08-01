use std::fs;
pub fn ls(input: &str) -> String {
    let mut output = String::new();
    let args: Vec<&str> = input.split_whitespace().collect();
    
    // Determine the directory to list
    let dir_path = if args.len() > 1 {
        args[1]  // Use provided path
    } else {
        "."      // Use current directory if no path provided
    };
    
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            let mut items = Vec::new();
            
            // Collect all entries
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
            
            // Sort the items alphabetically
            items.sort();
            
            // Format output (simple column format)
            for item in items {
                output.push_str(&format!("{}\n", item));
            }
            
            if output.is_empty() {
                // Directory is empty
                output.push_str("");
            }
        }
        Err(e) => {
            output.push_str(&format!("ls: {}: {}\n", dir_path, e));
        }
    }
    
    output
}