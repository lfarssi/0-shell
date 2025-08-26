use std::fs;
use std::path::Path;

pub fn mv(args: &[String]) -> String {
    if args.len() < 2 {
        return "mv: missing file operand".to_string();
    }

    let sources = &args[..args.len() - 1]; 
    let destination = &args[args.len() - 1];
    let dest_path = Path::new(destination);

    if sources.len() > 1 && (!dest_path.exists() || !dest_path.is_dir()) {
        return format!("mv: target '{}' is not a directory", destination);
    }

    let mut messages = Vec::new();

    for source in sources {
        let src_path = Path::new(source);
        let dest_file = if dest_path.is_dir() {
            dest_path.join(src_path.file_name().unwrap())
        } else {
            dest_path.to_path_buf()
        };

        match fs::rename(src_path, &dest_file) {
            Ok(_) => {}
            Err(e) => messages.push(format!("mv: cannot move '{}': {}", source, e)),
        }
    }

    if messages.is_empty() {
        String::new()
    } else {
        messages.join("\n")
    }
}
