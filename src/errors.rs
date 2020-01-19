use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GrmError {
    CreateRepositoryFailed(String),
    RemoveRepositoryFailed(String),
    RepositoryAlreadyExists(String),
    RepositoriesListEmpty,
    RepositoryNotFound(String),
    Database,
    RunPendingMigrationsFailed,
}

impl fmt::Display for GrmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GrmError::CreateRepositoryFailed(ref repo) => {
                write!(f, "Unable to save the repository {}", repo)
            }
            GrmError::RemoveRepositoryFailed(ref repo) => {
                write!(f, "Unable to remove the repository {}", repo)
            }
            GrmError::RepositoryAlreadyExists(ref repo) => write!(f, "{} already exists", repo),
            GrmError::RepositoriesListEmpty => write!(
                f,
                "No repository found - try to add a path or to run `grm refresh`"
            ),
            GrmError::RepositoryNotFound(ref repo) => write!(f, "Unable to find {}", repo),
            GrmError::Database => write!(f, "An error occured while querying the database"),
            GrmError::RunPendingMigrationsFailed => write!(f, "GRM update failed"),
        }
    }
}

impl Error for GrmError {}
