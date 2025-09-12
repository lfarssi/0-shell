
use std::path::Path;
use std::fs;


pub fn cp(args: &[String]) -> String {
    if args.len() < 2 {
        return "cp: missing file operand".to_string();
    }

    let sources = &args[..args.len() - 1]; // all but last = sources
    let destination = &args[args.len() - 1]; // last = destination
    let dest_path = Path::new(destination);

    // If multiple sources, destination must be a directory
    if sources.len() > 1 && (!dest_path.exists() || !dest_path.is_dir()) {
        return format!("cp: target '{}' is not a directory", destination);
    }

    let mut messages = Vec::new();

    for source in sources {
        let src_path = Path::new(source);
        let dest_file = if dest_path.is_dir() {
            dest_path.join(src_path.file_name().unwrap())
        } else {
            dest_path.to_path_buf()
        };

        match fs::copy(src_path, &dest_file) {
            Ok(_) => {}
            Err(e) => messages.push(format!("cp: cannot copy '{}': {}", source, e)),
        }
    }

    if messages.is_empty() {
        String::new()
    } else {
        messages.join("\n")
    }
}
