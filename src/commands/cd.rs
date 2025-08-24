use std::env;
use std::path::Path;

pub fn cd(args: &[String]) -> String {
     let target = if args.is_empty() {
        match env::var("HOME") {
            Ok(home) => home,
            Err(_) => match env::var("USERPROFILE") {
                Ok(home) => home,
                Err(_) => return "cd: HOME directory not found\n".to_string(),
            },
        }
    } else {
        args[0].clone()
    };

    change_directory(&target)
}

fn change_directory(path: &str) -> String {
    let target = Path::new(path);

    if !target.exists() {
        return format!("cd: {}: No such file or directory\n", path);
    }

    if !target.is_dir() {
        return format!("cd: {}: Not a directory\n", path);
    }

    match env::set_current_dir(target) {
        Ok(_) => String::new(),
        Err(e) => format!("cd: {}: {}\n", path, e),
    }
}