use std::{io::{self, Write}};

pub fn welcome()->Result<usize, io::Error>{
    let orange = "\x1b[38;5;208m";
    let reset = "\x1b[0m";
    let message = r#"

         _______         _______           _______  _        _       
        (  __   )       (  ____ \|\     /|(  ____ \( \      ( \      
        | (  )  |       | (    \/| )   ( || (    \/| (      | (      
        | | /   | _____ | (_____ | (___) || (__    | |      | |      
        | (/ /) |(_____)(_____  )|  ___  ||  __)   | |      | |      
        |   / | |             ) || (   ) || (      | |      | |      
        |  (__) |       /\____) || )   ( || (____/\| (____/\| (____/\
        (_______)       \_______)|/     \|(_______/(_______/(_______/

                        welcome to our shell
    "#;
    
    io::stdout().write(format!("{orange}{message}{reset}\n").as_bytes())

}
