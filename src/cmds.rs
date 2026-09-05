#![warn(dead_code)]
#![warn(unused_variables)]
#![warn(unused_imports)]

use std::path::Path;
use std::fs;
use rusqlite::{Connection, params};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

fn index_dir(path: &Path, conn: &Connection) {
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return, // Skip directories with permission issues
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            
            // Quietly index in the background without spamming text updates
            let _ = conn.execute(
                "INSERT OR IGNORE INTO entries (path, is_dir) VALUES (?1, ?2)",
                params![path.to_str().unwrap_or(""), path.is_dir() as i32]
            );

            if path.is_dir() {
                index_dir(&path, conn);
            }
        }
    }
}

pub fn init(path: &Path) {
    fs::create_dir_all(".indexter").unwrap();

    // Set up a steady background spinner that doesn't track file strings
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100)); // Smooth spin tick every 100ms
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
    );
    pb.set_message("Scanning directories...");

    let conn = Connection::open(".indexter/indexter.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries(
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL UNIQUE,
            is_dir INTEGER CHECK (is_dir IN (0, 1))
        )",
        [],
    ).unwrap();

    // Run the directory walker quietly
    index_dir(path, &conn);

    // Clear the spinner line and finish
    pb.finish_with_message("🎉 Indexing complete!");
}
