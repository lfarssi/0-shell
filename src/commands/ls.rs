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

    let Ok(entries) = fs::read_dir(path) else {
        return format!("ls: cannot access '{}': No such file or directory", path);
    };

    let mut items = Vec::new();
    if flags.contains('a') {
        items.push((".".to_string(), fs::metadata(path).ok()));
        items.push(("..".to_string(), fs::metadata(&format!("{}/..", path)).ok()));
    }
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.starts_with('.') || flags.contains('a') {
            items.push((name, entry.metadata().ok()));
        }
    }
    items.sort_by(|a, b| {
        let a_name = if a.0.starts_with('.') { &a.0[1..] } else { &a.0 };
        let b_name = if b.0.starts_with('.') { &b.0[1..] } else { &b.0 };
        a_name.to_lowercase().cmp(&b_name.to_lowercase())
    });
    if flags.contains('l') {
        let total: u64 = items
            .iter()
            .filter_map(|(_, meta)| meta.as_ref().map(|m| m.blocks() / 2))
            .sum();
        output.push_str(&format!("total {}\n", total));

        for (name, metadata) in items {
            format_long_entry(&mut output, &name, &metadata, &flags);
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
// ls -l kamla hia had l function:
fn format_long_entry(
    output: &mut String,
    name: &str,
    metadata: &Option<std::fs::Metadata>,
    flags: &str
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
                "{}{} {} {} {} {:>4} {} {}\n",
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
                display_name
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
    let utc_time: DateTime<Utc> = time.into();
    let local_time = utc_time.with_timezone(&Local) + chrono::Duration::hours(1);
    local_time.format("%b %d %H:%M").to_string()
}
// unsafe for libc 7it mafiha ti9a hhhhh
fn get_user_name(uid: u32) -> Option<String> {
    unsafe {
        let passwd = libc::getpwuid(uid);
        if passwd.is_null() {
            return None;
        }

        let name_ptr = (*passwd).pw_name;
        if name_ptr.is_null() {
            return None;
        }

        CStr::from_ptr(name_ptr).to_str().ok().map(String::from)
    }
}

fn get_group_name(gid: u32) -> Option<String> {
    unsafe {
        let group = libc::getgrgid(gid);
        let name_ptr = (*group).gr_name;
        if name_ptr.is_null() {
            return None;
        }

        CStr::from_ptr(name_ptr).to_str().ok().map(String::from)
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
