use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use db::*;
use glob::glob;
use std::result::Result;
use std::path::PathBuf;
use models::{Repository, NewRepository};

#[derive(Debug)]
pub struct GitRepo {
    pattern: String
}

impl GitRepo {
    pub fn new(pattern: String) -> GitRepo {
        GitRepo {
            pattern: String::from(pattern)
        }
    }

    pub fn add(&self, path: String) {
        let search = path.to_string() + &self.pattern.to_string();
        let repositories = self.search(search);

        &self.create_repositories(&repositories);
    }

    fn search(&self, pattern: String) -> Vec<NewRepository> {
        let mut repositories: Vec<NewRepository> = Vec::new();

        for path in glob(&pattern).unwrap().filter_map(Result::ok) {
            let str_path = path.as_path().to_str().unwrap();
            let parent = path.parent().unwrap();
            let name = parent.iter().last().unwrap().to_str().unwrap();
            let splitted_path: Vec<&str> = str_path.split(".git").collect();
            let path = splitted_path[0];

            let new_repo = NewRepository::new(String::from(name), String::from(path));

            repositories.push(new_repo);
        }

        repositories
    }

    // -> Vec<Repository>
    // TODO move to models
    fn create_repositories<'a>(&self, new_repositories: &'a Vec<NewRepository>) -> usize {
        use schema::repositories;

        let connection = establish_connection();

        diesel::insert(new_repositories)
            .into(repositories::table)
            .execute(&connection)
            .expect("Error saving repositories")
    }
}