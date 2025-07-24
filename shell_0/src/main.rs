mod parsing;
mod commands;
fn main() {
   parsing::welcome::welcome();
loop {
        let result = parsing::input::reading_input();
        if result.trim().len()==0 {
            continue;
        }
        println!("{}", result);
    }
}
