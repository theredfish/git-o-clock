use rusqlite::Connection;
use rusqlite::Error;

pub fn initialize_connection() -> Result<Connection, Error> {
    let conn = Connection::open("grm_db")?;
    Ok(conn)
}

// TODO
pub fn create_repository_table() {
    println!("Create repo table");
}

