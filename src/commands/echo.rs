pub fn echo(input: &str) -> String {
    let args = input.strip_prefix("echo").unwrap_or("").trim_start();
    if (args.starts_with('"') && args.ends_with('"')) ||
       (args.starts_with('\'') && args.ends_with('\'')) {
        args[1..args.len() - 1].to_string()
    } else {
        args.to_string()
    }
}