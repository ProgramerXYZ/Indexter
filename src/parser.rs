#![allow(unused)]

use std::{ops::Index, vec};

enum CommandKind {
    Index,
    Search
}

struct Cmd {
    option : Option,
}


fn parser(x: &str) -> Vec<&str> {
     x.split_whitespace().collect()
}

fn detect_command(whole_cmd: &Vec<&str>)-> Result<Cmd, String>{
    
    match whole_cmd.get(1) {
        Some(&"index") =>  Ok(Cmd { option: CommandKind::Index }),
        Some(&"search") => Ok(Cmd { option: CommandKind::Search }),
        Some(cmd) => Err(format!("Unknown command '{}'", cmd)),
        None => Err(String::from("No command provided")),
    }
    
}