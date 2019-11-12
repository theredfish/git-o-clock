use crate::db::schema::repositories;
use crate::db::SqliteDatabase;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use globwalk::DirEntry;
use std::{convert::TryFrom, error, fmt};

#[derive(Queryable, Debug)]
pub struct Repository {
    pub name: String,
    pub path: String,
}

#[derive(Insertable, Debug)]
#[table_name = "repositories"]
pub struct NewRepository {
    pub name: String,
    pub path: String,
}

#[derive(Debug)]
pub enum NewRepositoryError {
    Io(std::io::Error),
    WalkPath,
}

impl fmt::Display for NewRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot create the repository for this path")
    }
}

impl error::Error for NewRepositoryError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl From<std::io::Error> for NewRepositoryError {
    fn from(err: std::io::Error) -> NewRepositoryError {
        NewRepositoryError::Io(err)
    }
}

impl NewRepository {
    pub fn new(name: String, path: String) -> NewRepository {
        NewRepository { name, path }
    }
}

impl TryFrom<DirEntry> for NewRepository {
    type Error = NewRepositoryError;

    fn try_from(dir_entry: DirEntry) -> Result<Self, Self::Error> {
        let dir_entry_path = dir_entry.path();
        let absolute_path = dunce::canonicalize(&dir_entry_path)?;
        let parent = absolute_path.parent().ok_or(NewRepositoryError::WalkPath)?;

        let name = parent
            .iter()
            .last()
            .ok_or(NewRepositoryError::WalkPath)?
            .to_str()
            .ok_or(NewRepositoryError::WalkPath)?;

        let path = parent.to_str().ok_or(NewRepositoryError::WalkPath)?;

        Ok(NewRepository::new(String::from(name), String::from(path)))
    }
}

pub fn create_repository(new_repository: &NewRepository) -> Result<Repository, Error> {
    use crate::db::schema::repositories::dsl::*;
    let connection = SqliteDatabase::new().conn;

    diesel::insert_into(repositories)
        .values(new_repository)
        .execute(&connection)?;

    // Get the last inserted repository
    // get_result isn't implemented with SQLite
    repositories.find(&new_repository.name).first(&connection)
}

pub fn get_repositories() -> Result<Vec<Repository>, Error> {
    use crate::db::schema::repositories::dsl::*;
    let connection = SqliteDatabase::new().conn;

    repositories.load::<Repository>(&connection)
}

pub fn get_repository(repo_name: String) -> Result<Repository, Error> {
    use crate::db::schema::repositories::dsl::*;
    let connection = SqliteDatabase::new().conn;

    repositories.find(repo_name).first(&connection)
}

pub fn remove_repository(repo_name: String) -> Result<usize, Error> {
    use crate::db::schema::repositories::dsl::*;
    let connection = SqliteDatabase::new().conn;

    diesel::delete(repositories)
        .filter(name.eq(repo_name))
        .execute(&connection)
}
