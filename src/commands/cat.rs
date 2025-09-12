use std::fs;
use std::io::{self, BufRead, Write};

pub fn cat(args: &[String]) -> String {
    // Function to read stdin interactively
    fn interactive_stdin() {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut handle_out = stdout.lock();

        for line in stdin.lock().lines() {
            match line {
                Ok(l) => {
                    // Print immediately
                    writeln!(handle_out, "{}", l).unwrap();
                }
                Err(e) => {
                    writeln!(handle_out, "cat: stdin: {}", e).unwrap();
                    break;
                }
            }
        }
    }

    // If no arguments → read from stdin interactively
    if args.is_empty() {
        interactive_stdin();
        return String::new(); // Return empty string
    }

    for (index, filename) in args.iter().enumerate() {
        if filename == "-" {
            interactive_stdin();
        } else {
            match fs::read_to_string(filename) {
                Ok(content) => print!("{}", content),
                Err(e) => eprintln!("cat: {}: {}", filename, e),
            }
        }
    }

    String::new()
}
