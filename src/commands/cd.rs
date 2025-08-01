use std::env;
use std::path::Path;

pub fn cd(input: &str) -> String {
    let args: Vec<&str> = input.split_whitespace().collect();
   
    let target_dir = if args.len() > 1 {
        args[1] 
    } else {
     
        match env::var("HOME") {
            Ok(home) => return change_directory(&home),
            Err(_) => {
               
                match env::var("USERPROFILE") {
                    Ok(home) => return change_directory(&home),
                    Err(_) => return "cd: HOME directory not found\n".to_string(),
                }
            }
        }
    };
  
}


fn change_directory(path: &str) -> String {
    let target_path = Path::new(path);
    
    if !target_path.exists() {
        return format!("cd: {}: No such file or directory\n", path);
    }
    
    if !target_path.is_dir() {
        return format!("cd: {}: Not a directory\n", path);
    }

    match env::set_current_dir(target_path) {
        Ok(_) => String::new(),
        Err(e) => format!("cd: {}: {}\n", path, e),
    }
}