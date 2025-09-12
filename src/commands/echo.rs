
pub fn echo(input: &[String]) -> String {
    if input.is_empty() {
        println!();
        return "".to_string();
    } 
    input.join(" ")
}