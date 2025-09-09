use std::fs;

pub fn cat(args: &[String]) -> String {
    if args.is_empty() {
        return String::from("cat: missing operand");
    }

    let mut output = String::new();

    for (index, filename) in args.iter().enumerate() {
        match fs::read_to_string(filename) {
            Ok(content) => output.push_str(&content);
            Err(e) => {
                output.push_str(&format!("cat: {}: {}", filename, e));
            }
        }
        if index < args.len() - 1 {
            output.push('\n');
        }
    }

    output
}
