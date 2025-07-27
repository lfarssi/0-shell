use std::fs;


pub fn cat(input: &str) -> String {
    let mut output = String::new();
    let files: Vec<&str> = input.split_whitespace().collect();
    
    for filename in files.iter().skip(1) {
        match fs::read_to_string(filename) {
            Ok(content) => output.push_str(&content),
            Err(e) => {
                output.push_str(&format!("cat: {}: {}\n", filename, e));
            }
        }
    }

    output
}

