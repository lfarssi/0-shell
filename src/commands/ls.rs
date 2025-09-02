use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path; // needed for checking executables on Unix

pub fn ls(args: &[String]) -> String {
    let mut output = String::new();

    let show_all = args.contains(&"-a".to_string());
    let long_format = args.contains(&"-l".to_string());
    let classify = args.contains(&"-F".to_string());

    let targets: Vec<&str> = {
        let filtered: Vec<&String> = args
            .iter()
            .filter(|s| *s != "-a" && *s != "-l" && *s != "-F")
            .collect();
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
            let mut name = target.to_string();
            if classify {
                if let Ok(m) = fs::symlink_metadata(path) {
                    name.push_str(&suffix_for(&m));
                }
            }
            output.push_str(&format!("{}\n", name));
            continue;
        }

        match fs::read_dir(path) {
            Ok(entries) => {
                let mut items = Vec::new();
                for entry in entries.flatten() {
                    let mut name = entry.file_name().to_string_lossy().to_string();

                    if !show_all && name.starts_with('.') {
                        continue;
                    }

                    let item_path = entry.path();

                    if long_format {
                        match fs::symlink_metadata(&item_path) {
                            Ok(m) => {
                                let file_type = {
                                    let m2 = m.file_type();
                                    if m2.is_dir() {
                                        "d"
                                    } else if m2.is_file() {
                                        "-"
                                    } else if m2.is_symlink() {
                                        "l"
                                    } else {
                                        "?"
                                    }
                                };
                                if classify {
                                    name.push_str(&suffix_for(&m));
                                }

                                let size = m.len();
                                let modified = m.modified().ok();
                                let modified_str = match modified {
                                    Some(time) => {
                                        let datetime: chrono::DateTime<chrono::Local> = time.into();
                                        datetime.format("%Y-%m-%d %H:%M").to_string()
                                    }
                                    None => String::from("??????????????"),
                                };
                                let perms = permissions_string(&m);

                                items.push(format!(
                                    "{}{} {:>10} {} {}",
                                    file_type,perms, size, modified_str, name
                                ));
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

fn suffix_for(m: &fs::Metadata) -> String {
    let ft = m.file_type();
    if ft.is_dir() {
        "/".to_string()
    } else if ft.is_symlink() {
        "@".to_string()
    } else if ft.is_file() && (m.permissions().mode() & 0o111 != 0) {
        "*".to_string()
    } else {
        "".to_string()
    }
}
fn permissions_string(metadata: &fs::Metadata) -> String {
    let mode = metadata.permissions().mode();
    let mut perms = String::new();

    // Owner
    perms.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    perms.push(if mode & 0o100 != 0 { 'x' } else { '-' });

    // Group
    perms.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    perms.push(if mode & 0o010 != 0 { 'x' } else { '-' });

    // Others
    perms.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    perms.push(if mode & 0o001 != 0 { 'x' } else { '-' });

    perms
}
