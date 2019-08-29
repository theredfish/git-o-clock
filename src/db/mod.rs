embed_migrations!("migrations");

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::RunMigrationsError;
use std::env;
use std::io::stdout;
use std::path::Path;
use std::process;

pub mod models;
pub mod schema;

const DB_FILE: &str = "grm.sqlite3";

pub fn establish_connection() -> SqliteConnection {
    let exe_path = match env::current_exe() {
        Ok(exe_path) => exe_path,
        Err(e) => {
            eprintln!("Failed to find the executable : {}", e);
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

    let key = "DATABASE_URL";
    let db_file = exe_dir.join(DB_FILE);

    if !Path::new(&db_file).exists() {
        eprintln!("Database file {} not found", DB_FILE);
        process::exit(1);
    }

    env::set_var(key, exe_dir.join(DB_FILE));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_pending_migrations() -> Result<(), RunMigrationsError> {
    let connection = establish_connection();
    embedded_migrations::run_with_output(&connection, &mut stdout())
}
