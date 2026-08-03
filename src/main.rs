mod parser;
use std::{env, process::Command};
use std::path::PathBuf;

use crate::parser::{Cmd, CommandKind};

fn inp() -> Vec<String> {
    env::args().skip(1).collect()
}

fn main() {
    let input = inp();
    
    
    let which_command: Result<CommandKind, String> = parser::detect_command(&input);
    let cmd = Cmd {
        command,
        path,
        current_path,
    };
    match which_command {
        Ok(CommandKind::Index) => {cmd.command=CommandKind::Index}
        Ok(CommandKind::Search) => {cmd.command=CommandKind::Search}
        Err(err) => {eprintln!("{}", err)}
        
    }
    cmd.current_path= std::env::current_dir();
    
    
        
}
