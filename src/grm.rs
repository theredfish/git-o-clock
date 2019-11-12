use crate::config::Config;
use crate::db::{models::*, SqliteDatabase};
use crate::errors::GrmError;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;
use globwalk;
use std::convert::TryFrom;
use std::path::Path;
use std::process;

pub struct Grm {
    config: Config,
}

impl Grm {
    pub fn new(config: Config) -> Self {
        let db = SqliteDatabase::new();

        // Run pending migrations - used to initialize and update the SQLite database.
        // TODO : prompt a question to know if the user wants to apply the update.
        if db.run_pending_migrations().is_err() {
            eprintln!("{}", GrmError::RunPendingMigrationsFailed);
            process::exit(1);
        };

        Grm { config }
    }

    pub fn add(self, path: String) {
        let search = self.search(path.as_str());

        for repo in search {
            match create_repository(&repo) {
                Ok(inserted_repo) => println!("-> {}", inserted_repo.name),
                Err(DatabaseError(UniqueViolation, _)) => {
                    eprintln!("{}", GrmError::RepositoryAlreadyExists(repo.name))
                }
                Err(_) => eprintln!("{}", GrmError::CreateRepositoryFailed(repo.name)),
            };
        }
    }

    // TODO : see how to handle the query error (e.g a connection error)
    // which is different of a non-existinf entry in the database
    pub fn goto(self, repo_name: String) {
        let repository_name = repo_name.clone();

        match get_repository(repo_name) {
            Ok(repo) => println!("{}", Path::new(&repo.path).display()),
            Err(_) => eprintln!("{}", GrmError::RepositoryNotFound(repository_name)),
        }
    }

    pub fn list(self) {
        match get_repositories() {
            Ok(repos) => {
                if !repos.is_empty() {
                    for repo in repos {
                        println!("{}", repo.name)
                    }
                } else {
                    eprintln!("{}", GrmError::RepositoriesListEmpty)
                }
            }
            Err(e) => eprintln!("{} - {}", GrmError::Database, e),
        }
    }

    pub fn rm(self, repo_name: String) {
        let repository_name = repo_name.clone();
        match remove_repository(repo_name) {
            Ok(count) => println!("{} project(s) removed", count),
            Err(_) => eprintln!("{}", GrmError::RemoveRepositoryFailed(repository_name)),
        }
    }

    fn search(self, path: &str) -> Vec<NewRepository> {
        println!("Searching for git repositories (it may take a few seconds).");
        let mut patterns = vec![".git".to_string()];

        if let Some(ignored_folders) = self.config.ignored_folders {
            for folder in ignored_folders {
                let ignored_folder = "!".to_string() + &folder;
                patterns.push(ignored_folder);
            }
        }

        let repositories: Vec<NewRepository> =
            globwalk::GlobWalkerBuilder::from_patterns(path.to_string(), &patterns)
                .max_depth(self.config.max_depth_search)
                .build()
                .unwrap()
                .filter_map(Result::ok)
                .map(NewRepository::try_from)
                .filter_map(|result| match result {
                    Ok(res) => Some(res),
                    Err(err) => {
                        eprintln!("{} : {}", path.to_string(), err);
                        None
                    }
                })
                .collect();

        repositories
    }
}
