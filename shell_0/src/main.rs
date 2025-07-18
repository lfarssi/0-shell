mod parsing;
fn main() {
   parsing::welcome::welcome();
loop {
        let result = parsing::input::reading_input();
        println!("{}", result);
    }
}
