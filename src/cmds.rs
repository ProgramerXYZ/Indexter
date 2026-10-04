#![warn(dead_code)]
#![warn(unused_variables)]
#![warn(unused_imports)]

use std::path::{Path, PathBuf};
use std::fs;
use rusqlite::{Connection, params};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

fn index_dir(path: &Path, conn: &Connection, pb: &ProgressBar) {
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return, // Skip directories with permission issues
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(error) => {
                    eprintln!("Could not read metadata: {}", error);
                    std::process::exit(1);
                }
            };
            
            let size = metadata.len() as i64;
            
            let last_modified: i64 = match metadata.modified() {
                Ok(time) => {
                    match time.duration_since(std::time::UNIX_EPOCH) {
                        Ok(duration) => duration.as_secs() as i64,
                        Err(error) => {
                            eprintln!("Could not convert modification time: {}", error);
                            std::process::exit(1);
                        }
                    }
                }
                Err(error) => {
                    eprintln!("Could not get modification time: {}", error);
                    std::process::exit(1);
                }
            };
            
            // This updates the {msg} token right next to the spinning wheel
            if let Some(path_str) = path.to_str() {
                pb.set_message(format!("Indexing: {}", path_str));
            }

            let path_dir = path.is_dir();

            let _ = conn.execute(
                "INSERT OR IGNORE INTO entries (path, is_dir, size, last_modified) VALUES (?1, ?2,?3, ?4)",
                params![path.to_str().unwrap_or(""), path_dir.clone() as i32, size, last_modified  ]
            );

            if path_dir {
                index_dir(&path, conn, pb);
            }
        }
    }
}

pub fn init(path: &Path) {
    fs::create_dir_all(".indexter").unwrap();

    // Create the loading spinner
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
    );
    pb.set_message("Preparing database...");

    let conn = Connection::open(".indexter/indexter.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries(
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL UNIQUE,
            is_dir INTEGER CHECK (is_dir IN (0, 1)),
            size INTEGER,
            last_modified INTEGER NOT NULL
        )",
        [],
    ).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS index_info(
            root_path TEXT NOT NULL
        )",
        [],
    ).unwrap();
    conn.execute(
        "INSERT INTO index_info (root_path) VALUES (?1)",
        params![path.to_str().unwrap_or("")],
    ).unwrap();

    // Pass the spinner into the recursive walker to update the text inline
    index_dir(path, &conn, &pb);

    // Clear everything out and display the final message on that line
    pb.finish_with_message("🎉 Indexing complete!");
}