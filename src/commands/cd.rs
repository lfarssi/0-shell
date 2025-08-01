use std::env;
use std::path::Path;

pub fn cd(input: &str) -> String {
    let args: Vec<&str> = input.split_whitespace().collect();
   
    let target_dir = if args.len() > 1 {
        args[1] 
    } else {
     
       
    };
    
 
}

