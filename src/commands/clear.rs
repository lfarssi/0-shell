pub fn clear() -> String {
    // ANSI escape code to clear screen and move cursor to top-left
    "\x1B[2J\x1B[H".to_string()
}
