use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;
use std::time::{ SystemTime, UNIX_EPOCH };
use chrono::{ DateTime, Local, Utc };
use std::ffi::CStr;

pub fn ls(input: &str) -> String {
    let mut output = String::new();
    let args: Vec<&str> = input.split_whitespace().skip(1).collect();
    let mut flags = String::new();
    let mut path = ".";
    for arg in &args {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                if !"laF".contains(c) {
                    return format!("ls: invalid option -- '{}'", c);
                }
                flags.push(c);
            }
        } else {
            path = arg;
        }
    }
    match fs::metadata(path) {
        Ok(meta) => {
            if meta.is_file() {
                let name = std::path::Path
                    ::new(path)
                    .file_name()
                    .unwrap_or_else(|| std::ffi::OsStr::new(path))
                    .to_string_lossy()
                    .to_string();

                if flags.contains('l') {
                    format_long_entry_dynamic(&mut output, &name, &Some(meta), &flags, 1, 1, 1, 1);
                    if output.ends_with('\n') {
                        output.pop();
                    }
                } else {
                    let display_name = if flags.contains('F') && meta.is_dir() {
                        format!("{}/", name)
                    } else {
                        name
                    };
                    output.push_str(&display_name);
                }
                return output;
            }
        }
        Err(e) => {
            return format!("ls: cannot access '{}': {}", path, match e.kind() {
                std::io::ErrorKind::NotFound => "No such file or directory".to_string(),
                std::io::ErrorKind::PermissionDenied => "Permission denied".to_string(),
                _ => e.to_string(),
            });
        }
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(e) => {
            return format!("ls: cannot access '{}': {}", path, match e.kind() {
                std::io::ErrorKind::NotFound => "No such file or directory".to_string(),
                std::io::ErrorKind::PermissionDenied => "Permission denied".to_string(),
                _ => e.to_string(),
            });
        }
    };

    let mut items = Vec::new();
    if flags.contains('a') {
        items.push((".".to_string(), fs::metadata(path).ok()));
        items.push(("..".to_string(), fs::metadata(&format!("{}/..", path)).ok()));
    }

    for entry in entries {
        match entry {
            Ok(entry) => {
                let name = entry.file_name().to_string_lossy().to_string();
                if !name.starts_with('.') || flags.contains('a') {
                    items.push((name, entry.metadata().ok()));
                }
            }
            Err(e) => {
                output.push_str(
                    &format!("ls: cannot access '{}': {}\n", path, match e.kind() {
                        std::io::ErrorKind::PermissionDenied => "Permission denied".to_string(),
                        _ => e.to_string(),
                    })
                );
            }
        }
    }
    items.sort_by(|a, b| {
        match (&a.0[..], &b.0[..]) {
            (".", _) => std::cmp::Ordering::Less,
            (_, ".") => std::cmp::Ordering::Greater,
            ("..", _) if a.0 != "." => std::cmp::Ordering::Less,
            (_, "..") if b.0 != "." => std::cmp::Ordering::Greater,
            _ => {
                let a_name = if a.0.starts_with('.') && a.0 != "." && a.0 != ".." {
                    &a.0[1..]
                } else {
                    &a.0
                };
                let b_name = if b.0.starts_with('.') && b.0 != "." && b.0 != ".." {
                    &b.0[1..]
                } else {
                    &b.0
                };
                a_name.to_lowercase().cmp(&b_name.to_lowercase())
            }
        }
    });
    if flags.contains('l') {
        let total: u64 = items
            .iter()
            .filter_map(|(_, meta)| meta.as_ref().map(|m| m.blocks() / 2))
            .sum();
        output.push_str(&format!("total {}\n", total));
        let max_nlinks_width = items
            .iter()
            .filter_map(|(_, meta)| meta.as_ref())
            .map(|m| m.nlink().to_string().len())
            .max()
            .unwrap_or(1);

        let max_user_width = items
            .iter()
            .filter_map(|(_, meta)| meta.as_ref())
            .map(|m|
                get_user_name(m.uid())
                    .unwrap_or_else(|| m.uid().to_string())
                    .len()
            )
            .max()
            .unwrap_or(1);

        let max_group_width = items
            .iter()
            .filter_map(|(_, meta)| meta.as_ref())
            .map(|m|
                get_group_name(m.gid())
                    .unwrap_or_else(|| m.gid().to_string())
                    .len()
            )
            .max()
            .unwrap_or(1);

        let max_size_width = items
            .iter()
            .filter_map(|(_, meta)| meta.as_ref())
            .map(|m| m.len().to_string().len())
            .max()
            .unwrap_or(1);

        for (name, metadata) in items {
            format_long_entry_dynamic(
                &mut output,
                &name,
                &metadata,
                &flags,
                max_nlinks_width,
                max_user_width,
                max_group_width,
                max_size_width
            );
        }
        if output.ends_with('\n') {
            output.pop();
        }
    } else {
        for (name, metadata) in items {
            let is_dir = metadata.as_ref().map_or(false, |m| m.is_dir());
            format_short_entry(&mut output, &name, is_dir, &flags);
        }
        if output.ends_with(' ') {
            output.pop();
        }
    }

    output
}
// wlit kan7sb size dyal kol element f lista
fn format_long_entry_dynamic(
    output: &mut String,
    name: &str,
    metadata: &Option<std::fs::Metadata>,
    flags: &str,
    nlinks_width: usize,
    user_width: usize,
    group_width: usize,
    size_width: usize
) {
    if let Some(meta) = metadata {
        let user = get_user_name(meta.uid()).unwrap_or_else(|| meta.uid().to_string());
        let group = get_group_name(meta.gid()).unwrap_or_else(|| meta.gid().to_string());
        let display_name = if flags.contains('F') && meta.is_dir() {
            format!("{}/", name)
        } else {
            name.to_string()
        };
        output.push_str(
            &format!(
                "{}{} {:>width_nlinks$} {:width_user$} {:width_group$} {:>width_size$} {} {}\n",
                if meta.is_dir() {
                    "d"
                } else {
                    "-"
                },
                format_permissions(meta),
                meta.nlink(),
                user,
                group,
                meta.len(),
                format_time(meta.modified().unwrap_or(UNIX_EPOCH)),
                display_name,
                width_nlinks = nlinks_width,
                width_user = user_width,
                width_group = group_width,
                width_size = size_width
            )
        );
    } else {
        let display_name = if flags.contains('F') {
            format!("{}?", name)
        } else {
            name.to_string()
        };

        output.push_str(
            &format!(
                "?????????? {:>width_nlinks$} {:width_user$} {:width_group$} {:>width_size$} {} {}\n",
                "?",
                "?",
                "?",
                "?",
                "? ? ?:?",
                display_name,
                width_nlinks = nlinks_width,
                width_user = user_width,
                width_group = group_width,
                width_size = size_width
            )
        );
    }
}

fn format_short_entry(output: &mut String, name: &str, is_dir: bool, flags: &str) {
    if flags.contains('F') && is_dir {
        output.push_str(&format!("{}/ ", name));
    } else {
        output.push_str(&format!("{} ", name));
    }
}

fn format_time(time: SystemTime) -> String {
    use std::mem;

    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let timestamp = duration.as_secs() as libc::time_t;

    unsafe {
        let mut tm: libc::tm = mem::zeroed();
        let tm_ptr = libc::localtime_r(&timestamp, &mut tm);

        if !tm_ptr.is_null() && tm.tm_mon >= 0 && tm.tm_mon < 12 {
            let months = [
                "Jan",
                "Feb",
                "Mar",
                "Apr",
                "May",
                "Jun",
                "Jul",
                "Aug",
                "Sep",
                "Oct",
                "Nov",
                "Dec",
            ];
            let month = months[tm.tm_mon as usize];

            format!("{} {:2} {:02}:{:02}", month, tm.tm_mday, tm.tm_hour, tm.tm_min)
        } else {
            let utc_time: DateTime<Utc> = time.into();
            let local_time = utc_time.with_timezone(&Local);
            local_time.format("%b %d %H:%M").to_string()
        }
    }
}
// unsafe for libc 7it mafiha ti9a hhhhh
fn get_user_name(uid: u32) -> Option<String> {
    unsafe {
        let passwd = libc::getpwuid(uid);
        if passwd.is_null() {
            return None;
        }

        let passwd_ref = &*passwd;
        let name_ptr = passwd_ref.pw_name;
        if name_ptr.is_null() {
            return None;
        }

        match CStr::from_ptr(name_ptr).to_str() {
            Ok(name) => Some(name.to_string()),
            Err(_) => None,
        }
    }
}

fn get_group_name(gid: u32) -> Option<String> {
    unsafe {
        let group = libc::getgrgid(gid);
        if group.is_null() {
            return None;
        }

        let group_ref = &*group;
        let name_ptr = group_ref.gr_name;
        if name_ptr.is_null() {
            return None;
        }

        match CStr::from_ptr(name_ptr).to_str() {
            Ok(name) => Some(name.to_string()),
            Err(_) => None,
        }
    }
}
// kandn kayna method 7sn l permisstion wlkn for now ankhli hado
fn format_permissions(metadata: &std::fs::Metadata) -> String {
    let mode = metadata.permissions().mode();
    let mut perms = String::with_capacity(9);
    perms.push(if (mode & 0o400) != 0 { 'r' } else { '-' });
    perms.push(if (mode & 0o200) != 0 { 'w' } else { '-' });
    perms.push(if (mode & 0o100) != 0 { 'x' } else { '-' });
    perms.push(if (mode & 0o040) != 0 { 'r' } else { '-' });
    perms.push(if (mode & 0o020) != 0 { 'w' } else { '-' });
    perms.push(if (mode & 0o010) != 0 { 'x' } else { '-' });
    perms.push(if (mode & 0o004) != 0 { 'r' } else { '-' });
    perms.push(if (mode & 0o002) != 0 { 'w' } else { '-' });
    perms.push(if (mode & 0o001) != 0 { 'x' } else { '-' });

    perms
}
