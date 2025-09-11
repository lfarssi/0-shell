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

    for (_, target) in targets.iter().enumerate() {
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
                    output.push_str(&format!("{}\n", long_format_line(path, &m, &name)));
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
                let mut items: Vec<(String, std::path::PathBuf, Option<fs::Metadata>)> = Vec::new();
                if show_all {
                    let dot = path.join(".");
                    let dotdot = path.join("..");
                    items.push((
                        ".".to_string(),
                        dot.clone(),
                        fs::symlink_metadata(&dot).ok(),
                    ));
                    items.push((
                        "..".to_string(),
                        dotdot.clone(),
                        fs::symlink_metadata(&dotdot).ok(),
                    ));
                }
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if !show_all && name.starts_with('.') {
                        continue;
                    }
                    let item_path = entry.path();
                    println!("Reading entry: {:?}", item_path);
                    let meta = fs::symlink_metadata(&item_path).ok();
                    items.push((name, item_path, meta));
                }
                items.sort_by(|a, b| ls_cmp(&a.0, &b.0));
                // Calculate total blocks for long format
                if long_format {
                    // Sum 512-byte blocks, then convert to 1K blocks (like ls)
                    let total_blocks_512: u64 = items
                        .iter()
                        .filter_map(|(_,_ , meta)| meta.as_ref())
                        .map(|m| m.blocks() as u64)
                        .sum();
                    // Convert to 1K blocks, rounding up if needed
                    let total_blocks_1k = (total_blocks_512 + 1) / 2;
                    output.push_str(&format!("total {}\n", total_blocks_1k));
                }

                let mut short_names = Vec::new();
                for (name, item_path,meta) in &items {
                    let mut display_name = name.clone();
                    if let Some(m) = meta.as_ref() {
                        if classify {
                            // For symlinks: if executable, use *, else @
                            if m.file_type().is_symlink() && !long_format {
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
                            output.push_str(&format!(
                                "{}\n",
                                long_format_line(item_path, m, &display_name)
                            ));
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
                    let cols = if col_width == 0 {
                        1
                    } else {
                        term_width / col_width
                    };
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

// Custom sort: '.' and '..' first, then case-insensitive lexicographical order, symbols before letters
fn ls_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    // Special entries
    if a == "." && b != "." {
        return std::cmp::Ordering::Less;
    }
    if b == "." && a != "." {
        return std::cmp::Ordering::Greater;
    }
    if a == ".." && b != ".." {
        return std::cmp::Ordering::Less;
    }
    if b == ".." && a != ".." {
        return std::cmp::Ordering::Greater;
    }
    // Case-insensitive comparison
    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();
    match a_lower.cmp(&b_lower) {
        std::cmp::Ordering::Equal => a.cmp(b), // Lowercase before uppercase if equal ignoring case
        ord => ord,
    }
}

fn long_format_line(path: &Path, metadata: &fs::Metadata, name: &str) -> String {
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

    // For symlinks, append -> target (with suffix if needed)
    let mut display_name = name.to_string();
    if metadata.file_type().is_symlink() {
        if let Ok(target_path) = std::fs::read_link(path) {
            let target_str = target_path.to_string_lossy();
            // Try to get metadata for the target to determine suffix
            let mut target_display = target_str.to_string();
            if let Ok(target_meta) = std::fs::metadata(&target_path) {
                target_display.push_str(&suffix_for(&target_meta));
            }
            display_name.push_str(&format!(" -> {}", target_display));
        } else {
            println!("Could not read symlink target for {}", name);
        }
    }
    format!(
        "{}{} {:>2} {:<8} {:<8} {:>8} {} {}",
        file_type, perms, nlink, user, group, size, date_str, display_name
    )
}

fn suffix_for(m: &fs::Metadata) -> String {
    let ft = m.file_type();
    if ft.is_dir() {
        "/".to_string()
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
