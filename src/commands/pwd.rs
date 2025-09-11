pub fn pwd() -> String {
    // First, try the PWD environment variable
    if let Ok(pwd_env) = std::env::var("PWD") {
        return pwd_env;
    }

    // Fallback: try to get from filesystem
    std::env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "pwd: cannot access current directory".to_string())
}
