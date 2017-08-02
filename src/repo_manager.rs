use glob::glob;
use models::Repository;
use rusqlite::Connection;
use std::result::Result;

#[derive(Debug)]
pub struct GitRepo {
    conn: Connection,
    pattern: String
}

// TODO see how to correctly handle db connection (trait with common fn for models?)
impl GitRepo {
    pub fn new(conn: Connection, pattern: String) -> GitRepo {
        GitRepo {
            conn: conn,
            pattern: String::from(pattern)
        }
    }

    pub fn add(&self, path: String) {
        let search = path.to_string() + &self.pattern.to_string();
        let git_repo_list = self.search(search);

        for i in 0..git_repo_list.len() {
            git_repo_list[i].insert(&self.conn);
        }
    }

    // TODO see if it's possible to exclude folders (node_modules, ...)
    fn search(&self, search: String) -> Vec<Repository> {
        let mut repositories: Vec<Repository> = Vec::new();

        // TODO handle error cases for paths (parent is root path, bad path, ...)
        for path in glob(&search).unwrap().filter_map(Result::ok) {
            let str_path = path.as_path().to_str().unwrap();
            let parent = path.parent().unwrap();
            let name = parent.iter().last().unwrap().to_str().unwrap();

            let repo = Repository::new(String::from(name), String::from(str_path));
            repositories.push(repo);
        }

        repositories
    }
}