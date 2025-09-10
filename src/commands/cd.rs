use std::env;
use std::path::Path;

pub fn cd(args: &[String]) -> String {
    // Save current directory
    let current_dir = env::current_dir().unwrap().to_string_lossy().into_owned();

    // Determine target directory
    let target = if args.is_empty() {
        env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or_else(|_| {
            return "cd: No home directory found (HOME or USERPROFILE not set)".to_string();
        })
    } else {
        let arg = args[0].trim();
        if arg == "-" {
            env::var("OLDPWD").unwrap_or_else(|_| {
                return "cd: OLDPWD not set".to_string();
            })
        } else if arg == "~" {
            env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or_else(|_| {
                return "cd: No home directory found (HOME or USERPROFILE not set)".to_string();
            })
        } else {
            args[0].clone()
        }
    };

    // Attempt to change directory
    let result = change_directory(&target);

    // If successful, update OLDPWD
    if result.is_empty() {
        env::set_var("OLDPWD", current_dir);
    }

    result
}

pub fn change_directory(path: &str) -> String {
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
