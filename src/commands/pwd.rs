pub fn pwd() -> String {
    std::env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "Failed to get current directory".to_string())
}
