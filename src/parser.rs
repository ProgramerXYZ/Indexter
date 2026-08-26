
use std::path::PathBuf;

pub enum CommandKind {
    Index,
    Search
}
#[allow(unused)]
pub struct Cmd {
    pub command: CommandKind,
    pub path: PathBuf,
    pub current_path: Result<PathBuf,std::io::Error>,
}

// ------------has no use cause of env::args()------------
// fn parser(x: &str) -> Vec<&str> {
//      x.split_whitespace().collect()
// }

pub fn detect_command(whole_cmd: &[String])-> Result<CommandKind, String>{    
    match whole_cmd.get(1).map(|s| s.as_str()) {
        Some("index") =>  Ok(CommandKind::Index),
        Some("search") => Ok(CommandKind::Search),
        Some(cmd) => Err(format!("Unknown command '{}'", cmd)),
        None => Err(String::from("No command provided")),
    }
}
pub fn argument_reader(whole_cmd: &[String])-> PathBuf{
    let location_dir = PathBuf::from(&whole_cmd[2]);
    return location_dir;
}