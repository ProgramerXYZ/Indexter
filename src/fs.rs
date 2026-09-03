#![warn(dead_code)]
#![warn(unused_variables)]
#![warn(unused_imports)]

use std::path::{Path, PathBuf};
use std::fs;
use rusqlite::{Connection, params};





fn index_dir(path:&Path, conn: &Connection) {
    
    
    let entries = fs::read_dir(path).unwrap();
    std::fs::create_dir(".indexter").unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path= entry.path();

        conn.execute("INSERT INTO entries (path, is_dir) VALUES (?1, ?2)",
         params![path.to_str().unwrap(), path.is_dir() as i32]
        ).unwrap();


        if path.is_dir() {

            index_dir(&path , conn);
        }
        
    }
}
fn init_indexter(path: &Path) {
    fs::create_dir(".indexter").unwrap();

    let conn = Connection::open(".indexter/indexter.db").unwrap();

    conn.execute(
        "CREATE TABLE entries(
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL UNIQUE,
            is_dir INTEGER CHECK (is_dir IN (0, 1))
        )",
        [],
    ).unwrap();

    index_dir(path, &conn);
}


