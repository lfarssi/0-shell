pub fn echo(input :&str)->String{
   input.split_whitespace().skip(1).collect()
            
}