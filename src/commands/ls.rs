use chrono::{Datelike, TimeZone};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use users::{get_group_by_gid, get_user_by_uid};

pub fn ls(args: &[String]) -> String {
    let mut output = String::new();
    let mut show_all = false;
    let mut long_format = false;
    let mut classify = false;

    let mut targets: Vec<&str> = Vec::new();

    for arg in args {
        if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => show_all = true,
                    'l' => long_format = true,
                    'F' => classify = true,
                    _ => {
                        output.push_str(&format!("ls: invalid option -- '{}'\n", ch));
                    }
                }
            }
        } else {
            targets.push(arg.as_str());
        }
    }
    if targets.is_empty() {
        targets.push(".");
    }

    for (i, target) in targets.iter().enumerate() {
        let path = Path::new(target);
        
        if targets.len() > 1 {
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
                if show_all {
                    // Insert '.' and '..' at the start
                    let dot = path.join(".");
                    let dotdot = path.join("..");
                    items.push((".".to_string(), fs::symlink_metadata(&dot).ok()));
                    items.push(("..".to_string(), fs::symlink_metadata(&dotdot).ok()));
                }
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if !show_all && name.starts_with('.') {
                        continue;
                    }
                    let item_path = entry.path();
                    let meta = fs::symlink_metadata(&item_path).ok();
                    items.push((name, meta));
                }
                // sort list , ls we keep '.'  w '..' at the top
                if show_all {
                    let (special, mut rest): (Vec<_>, Vec<_>) = items.into_iter().partition(|(n,_)| n == "." || n == ".." );
                    let mut rest_sorted = rest;
                    rest_sorted.sort_by(|a, b| a.0.cmp(&b.0));
                    items = [special, rest_sorted].concat();
                } else {
                    items.sort_by(|a, b| a.0.cmp(&b.0));
                }
                // Calculate total blocks for long format
                if long_format {
                    let total_blocks: u64 = items.iter()
                        .filter_map(|(_, meta)| meta.as_ref())
                        .map(|m| m.blocks() as u64)
                        .sum();
                    output.push_str(&format!("total {}\n", total_blocks));
                }

                let mut short_names = Vec::new();
                for (name, meta) in &items {
                    let mut display_name = name.clone();
                    if let Some(m) = meta.as_ref() {
                        if classify {
                            // For symlinks: if executable, use *, else @
                            if m.file_type().is_symlink() {
                                if let Ok(target_meta) = std::fs::metadata(Path::new(&name)) {
                                    if target_meta.permissions().mode() & 0o111 != 0 {
                                        display_name.push('*');
                                    } else {
                                        display_name.push('@');
                                    }
                                } else {
                                    display_name.push('@');
                                }
                            } else {
                                display_name.push_str(&suffix_for(m));
                            }
                        }
                        // Add suffix for . and .. in long format with -F
                        // if long_format && classify && (name == "." || name == "..") {
                        //     if m.file_type().is_dir() {
                        //        // display_name.push('/');
                        //     }
                        // }
                        if long_format {
                            output.push_str(&format!("{}\n", long_format_line(m, &display_name)));
                        } else {
                            short_names.push(display_name);
                        }
                    } else if long_format {
                        output.push_str(&format!("?????????? {} {}\n", 0, name));
                    } else {
                        short_names.push(display_name);
                    }
                }
                // Print short format in columns (simple, space-separated)
                if !long_format {
                    let col_width = short_names.iter().map(|s| s.len()).max().unwrap_or(0) + 2;
                    let term_width = 80; // fallback if can't get terminal width
                    let cols = if col_width == 0 { 1 } else { term_width / col_width };
                    for (i, name) in short_names.iter().enumerate() {
                        output.push_str(name);
                        let is_last = i == short_names.len() - 1;
                        if (i + 1) % cols == 0 || is_last {
                            output.push('\n');
                        } else {
                            for _ in 0..(col_width - name.len()) {
                                output.push(' ');
                            }
                        }
                    }
                }
            }
            Err(e) => {
                output.push_str(&format!("ls: {}: {}\n", target, e));
            }
        }
    }
    if output.ends_with('\n') {
        output.pop();
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
