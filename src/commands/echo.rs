use std::process::Command;

pub fn echo(input: &[String]) -> String {
    let mut result = Vec::new();

    for arg in input {
        let mut expanded = String::new();
        let mut inside_bt = false;
        let mut cmd = String::new();

        for c in arg.chars() {
            if c == '`' {
                if inside_bt {
                    // closing backtick -> run the command
                    let parts: Vec<&str> = cmd.split_whitespace().collect();
                    if !parts.is_empty() {
                        let output = Command::new(parts[0])
                            .args(&parts[1..])
                            .output();

                        match output {
                            Ok(o) => {
                                let out_str =
                                    String::from_utf8_lossy(&o.stdout).trim().to_string();
                                expanded.push_str(&out_str);
                            }
                            Err(_) => expanded.push_str(&format!("`{}`", cmd)),
                        }
                    }
                    cmd.clear();
                }
                inside_bt = !inside_bt;
            } else if inside_bt {
                cmd.push(c);
            } else {
                expanded.push(c);
            }
        }

        // if still inside backticks (unclosed), keep them as text
        if inside_bt {
            expanded.push('`');
            expanded.push_str(&cmd);
        }

        result.push(expanded);
    }

    result.join(" ")
}
