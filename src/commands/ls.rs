use std::fs;
use std::path::Path;

pub fn ls(args: &[String]) -> String {
    let mut output = String::new();

    let show_all = args.contains(&"-a".to_string());
    let long_format = args.contains(&"-l".to_string());

    let targets: Vec<&str> = {
        let filtered: Vec<&String> = args.iter().filter(|s| *s != "-a" && *s != "-l").collect();
        if filtered.is_empty() {
            vec!["."]
        } else {
            filtered.iter().map(|s| s.as_str()).collect()
        }
    };

    for (i, target) in targets.iter().enumerate() {
        let path = Path::new(target);

        if targets.len() > 1 {
            if i > 0 {
                output.push('\n');
            }
            output.push_str(&format!("{}:\n", target));
        }

        if path.is_file() {
            output.push_str(&format!("{}\n", target));
            continue;
        }

        match fs::read_dir(path) {
            Ok(entries) => {
                let mut items = Vec::new();
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();

                    if !show_all && name.starts_with('.') {
                        continue;
                    }

                    let item_path = entry.path();

                    if long_format {
                        match fs::metadata(&item_path) {
                            Ok(m) => {
                                let size = m.len();
                                let modified = m.modified().ok();
                                let modified_str = match modified {
                                    Some(time) => {
                                        let datetime: chrono::DateTime<chrono::Local> = time.into();
                                        datetime.format("%Y-%m-%d %H:%M").to_string()
                                    }
                                    None => String::from("??????????????"),
                                };

                                items.push(format!("{:>10} {} {}", size, modified_str, name));
                            }
                            Err(_) => {
                                items.push(format!("{:>10} {} {}", 0, "??????????????", name));
                            }
                        }
                    } else {
                        items.push(name);
                    }
                }
                items.sort();
                for item in items {
                    output.push_str(&format!("{}\n", item));
                }
            }
            Err(e) => {
                output.push_str(&format!("ls: {}: {}\n", target, e));
            }
        }
    }

    output
}
