#![warn(unused_variables)]
mod parser;
mod fs;

use std::env; 

use crate::parser::{CommandKind, argument_reader, Cmd};


fn inp() -> Vec<String> {
    env::args().skip(1).collect()
}


fn main() {
    let input = inp();
    
    let which_command: Result<CommandKind, String> = parser::detect_command(&input);

    
    let command = { match which_command {
        Ok(CommandKind::Index) => CommandKind::Index,
        Ok(CommandKind::Search) => CommandKind::Search,
        Err(err) => {eprintln!("{}", err);
        std::process::exit(1)}
    }};
    
        let cmd: Cmd= Cmd{
            command,
            path: argument_reader(&input),
            current_path : std::env::current_dir(),
        }; 

           


}