#![warn(unused_variables)]
mod parser;
mod fs;

use std::{env, path::{Path, PathBuf}}; 

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
        Ok(CommandKind::Init) => CommandKind::Init,
        Err(err) => {eprintln!("{}", err);
        std::process::exit(1)}
    }};
    
        let cmd: Cmd= Cmd{
            command,
            path: argument_reader(&input),
            current_path : std::env::current_dir(),
        }; 

        let r_path:PathBuf;

        if cmd.path.as_os_str().is_empty()  {
            r_path=PathBuf::from(".");
        }
        else if !cmd.path.as_os_str().is_empty() {
            r_path=cmd.path;
        }

        





           


}