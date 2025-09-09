mod parsing;
mod commands;
fn main() {
   if !parsing::welcome::welcome().is_ok(){
    return
   }
loop {
        let result = parsing::input::reading_input();
        
        if result.trim().len()==0 {
            continue;
        }
        println!("{}", result);
    }
}
