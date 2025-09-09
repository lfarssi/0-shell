use std::fs;
use std::io::{self, Read};

pub fn cat(args: &[String]) -> String {
    let mut output = String::new();

    // If no arguments → read from stdin
    if args.is_empty() {
        let mut buffer = String::new();
        match io::stdin().read_to_string(&mut buffer) {
            Ok(_) => output.push_str(&buffer),
            Err(e) => output.push_str(&format!("cat: stdin: {}", e)),
        }
        return output;
    }

    for (index, filename) in args.iter().enumerate() {
        if filename == "-" {
            // Handle stdin when "-" is passed
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => output.push_str(&buffer),
                Err(e) => output.push_str(&format!("cat: stdin: {}", e)),
            }
        } else {
            // Normal file reading
            match fs::read_to_string(filename) {
                Ok(content) => output.push_str(&content),
                Err(e) => output.push_str(&format!("cat: {}: {}", filename, e)),
            }
        }

        if index < args.len() - 1 {
            output.push('\n');
        }
    }

    output
}
