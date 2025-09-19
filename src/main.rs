extern crate chrono;
extern crate users;
mod parsing;
mod commands;
fn main() {
    if !parsing::welcome::welcome().is_ok() {
        return;
    }

    loop {
        match parsing::input::reading_input() {
            Some(result) if !result.trim().is_empty() => {
                println!("{}", result);
            }
            Some(_) => continue, // empty input → keep looping
            None => {
                println!(); // print newline like real shells on Ctrl+D
                break; // exit loop
            }
        }
    }
}
