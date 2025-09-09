use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use chrono::{DateTime, Local, TimeZone, Datelike};
use users::{get_user_by_uid, get_group_by_gid};

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
            if long_format {
                if let Ok(m) = fs::symlink_metadata(path) {
                    output.push_str(&format!("{}\n", long_format_line(&m, &name)));
                } else {
                    output.push_str(&format!("{}\n", name));
                }
            } else {
                output.push_str(&format!("{}\n", name));
            }
            continue;
        }

        match fs::read_dir(path) {
            Ok(entries) => {
                let mut items: Vec<(String, Option<fs::Metadata>)> = Vec::new();
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if !show_all && name.starts_with('.') {
                        continue;
                    }
                    let item_path = entry.path();
                    let meta = fs::symlink_metadata(&item_path).ok();
                    items.push((name, meta));
                }
                // Sort by name, case-sensitive, like real ls
                items.sort_by(|a, b| a.0.cmp(&b.0));
                for (mut name, meta) in items {
                    if long_format {
                        if let Some(m) = meta {
                            let mut display_name = name.clone();
                            if classify {
                                display_name.push_str(&suffix_for(&m));
                            }
                            output.push_str(&format!("{}\n", long_format_line(&m, &display_name)));
                        } else {
                            output.push_str(&format!("?????????? {} {}\n", 0, name));
                        }
                    } else {
                        output.push_str(&format!("{}\n", name));
                    }
                }
            }
            Err(e) => {
                output.push_str(&format!("ls: {}: {}\n", target, e));
            }
        }
    }

    output
}

fn long_format_line(metadata: &fs::Metadata, name: &str) -> String {
    let file_type = {
        let ft = metadata.file_type();
        if ft.is_dir() {
            'd'
        } else if ft.is_symlink() {
            'l'
        } else if ft.is_file() {
            '-'
        } else {
            '?'
        }
    };
    let perms = permissions_string(metadata);
    let nlink = metadata.nlink();
    let uid = metadata.uid();
    let gid = metadata.gid();
    let user = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().to_string())
        .unwrap_or(uid.to_string());
    let group = get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().to_string())
        .unwrap_or(gid.to_string());
    let size = metadata.len();
    let mtime = metadata.mtime();
    // Convert mtime to local time using chrono::Local
    let datetime = chrono::Local.timestamp(mtime, 0);
    let now = chrono::Local::now();
    // ls shows year if not this year, else HH:MM
    let date_str = if datetime.year() == now.year() {
        datetime.format("%b %e %H:%M").to_string()
    } else {
        datetime.format("%b %e  %Y").to_string()
    };
    format!(
        "{}{} {:>2} {:<8} {:<8} {:>8} {} {}",
        file_type, perms, nlink, user, group, size, date_str, name
    )
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
