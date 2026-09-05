
use std::path::PathBuf;

pub enum CommandKind {
    Init,
    Index,
    Search,
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
        Some("init") => Ok(CommandKind::Init),
        Some(cmd) => Err(format!("Unknown command '{}'", cmd)),
        None => Err(String::from("No command provided")),
    }
}
pub fn argument_reader(whole_cmd: &[String])-> PathBuf{
    let location_dir={match whole_cmd.get(2) {
        Some(path_str) => PathBuf::from(path_str),
        None => PathBuf::new(), // Returns an empty PathBuf so main.rs can handle it!
    }};
    return location_dir;
}