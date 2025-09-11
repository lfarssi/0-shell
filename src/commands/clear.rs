pub fn clear() -> String {
    // ANSI escape code to clear screen and move cursor to top-left
    "\x1Bc".to_string()
}
