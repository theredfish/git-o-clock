use rusqlite::Connection;

#[derive(Debug)]
pub struct Repository {
    pub name: String,
    pub path: String
}

impl Repository {
    pub fn new(name: String, path: String) -> Repository {
        Repository {
            name: name,
            path: path
        }
    }

    // TODO implementation + return created repo
    pub fn insert(&self, conn: &Connection) {
        println!("in insert");
    }

}