use crate::config::Config;
use crate::db::{models::*, SqliteDatabase};
use crate::errors::GrmError;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;
use globwalk;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};
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

    pub fn add(self, path: String) -> Vec<Result<Repository, GrmError>> {
        let search = self.search(path.as_str());

        search
            .iter()
            .map(|new_repo| match create_repository(new_repo) {
                Ok(repo) => Ok(repo),
                Err(DatabaseError(UniqueViolation, _)) => {
                    Err(GrmError::RepositoryAlreadyExists(new_repo.name.to_owned()))
                }
                Err(_) => Err(GrmError::CreateRepositoryFailed(new_repo.name.to_owned())),
            })
            .collect()
    }

    // TODO : see how to handle the query error (e.g a connection error)
    // which is different of a non-existing entry in the database
    pub fn location(self, repo_name: String) -> Result<PathBuf, GrmError> {
        let repository_name = repo_name.clone();

        match get_repository(repo_name) {
            Ok(repo) => Ok(Path::new(&repo.path).to_owned()),
            Err(_) => Err(GrmError::RepositoryNotFound(repository_name)),
        }
    }

    pub fn list(self) -> Result<Vec<Repository>, GrmError> {
        match get_repositories() {
            Ok(repos) => {
                if repos.is_empty() {
                    return Err(GrmError::RepositoriesListEmpty);
                }

                Ok(repos)
            }
            Err(_) => Err(GrmError::Database),
        }
    }

    pub fn rm(self, repo_name: String) -> Result<usize, GrmError> {
        let repository_name = repo_name.clone();
        match remove_repository(repo_name) {
            Ok(count) => {
                if count > 0 {
                    return Ok(count);
                }

                Err(GrmError::RepositoryNotFound(repository_name))
            }
            Err(_) => Err(GrmError::RemoveRepositoryFailed(repository_name)),
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
