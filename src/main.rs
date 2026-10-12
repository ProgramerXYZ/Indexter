#![warn(unused_variables)]
mod parser;
mod cmds;

use std::{env, path::{Path, PathBuf}}; 

use crate::parser::{CommandKind, argument_reader, Cmd};




fn input_() -> Vec<String> {
    env::args().collect()
}


fn main() {
    let input = input_();
     
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
        else{
            r_path=cmd.path;
        }

        let pass_path:PathBuf;
        if r_path.as_path() == Path::new(".") {
            // convert it
            pass_path = std::path::absolute(&r_path).unwrap();
        } else {
            pass_path = r_path;
        }

        // match cmd.command {
        //     CommandKind::Init => cmds::init(&r_path),
        //     CommandKind::Index 
        // }

        cmds::init(&pass_path);


           
        drop(pass_path);

}