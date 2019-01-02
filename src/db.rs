use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;
use std::path::Path;
use std::process;

pub fn establish_connection() -> SqliteConnection {
    let exe_path = match env::current_exe() {
        Ok(exe_path) => exe_path,
        Err(e) => {
            eprintln!("Failed to find the path to the executable : {}", e);
            process::exit(1);
        }
    };

    let exe_dir = match exe_path.parent() {
        Some(dir) => dir,
        None => {
            eprintln!("Failed to find the the path to the installation folder");
            process::exit(1);
        }
    };

    let key = "DATABASE_URL";
    let db_file = exe_dir.join("grm.sqlite3");

    if !Path::new(&db_file).exists() {
        eprintln!("Database file not found");
        process::exit(1);
    }

    env::set_var(key, exe_dir.join("grm.sqlite3"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
