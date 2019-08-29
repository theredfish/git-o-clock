embed_migrations!("migrations");

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::RunMigrationsError;
use std::env;
use std::fs::File;
use std::io::stdout;
use std::path::Path;
use std::process;

pub mod models;
pub mod schema;

const DB_FILE: &str = "grm.db";

pub struct SqliteDatabase {
    conn: SqliteConnection,
}

impl SqliteDatabase {
    pub fn new() -> Self {
        let exe_path = match env::current_exe() {
            Ok(exe_path) => exe_path,
            Err(e) => {
                eprintln!("Failed to get the executable path : {}", e);
                process::exit(1);
            }
        };

        let exe_dir = match exe_path.parent() {
            Some(dir) => dir,
            None => {
                eprintln!("Failed to find the installation folder");
                process::exit(1);
            }
        };

        let db_file = exe_dir.join(DB_FILE);
        let db_path = &db_file.to_str();

        if !Path::new(&db_file).exists() {
            println!("This a fresh installation : creating the database file.");

            if let Err(err) = File::create(&db_file) {
                panic!("Cannot create the database file. Error : {}", err);
            }
        }

        if let Some(db_path) = db_path {
            let conn = SqliteConnection::establish(db_path).unwrap_or_else(|_| {
                panic!("Cannot establish the connection to the database.");
            });

            SqliteDatabase { conn }
        } else {
            panic!("Cannot establish the connection to the database");
        }
    }

    pub fn run_pending_migrations(self) -> Result<(), RunMigrationsError> {
        embedded_migrations::run_with_output(&self.conn, &mut stdout())
    }
}
