use rusqlite::Connection;
use rusqlite::Error;

pub struct SQLite {
    pub conn: Connection,
}

impl SQLite {
    pub fn new() -> Result<SQLite, Error> {
        let conn = Connection::open("grm_db")?;

        Ok(SQLite {
            conn: conn,
        })
    }
}