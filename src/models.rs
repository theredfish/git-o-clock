embed_migrations!("migrations");

use crate::db::establish_connection;
use crate::schema::repositories;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use diesel_migrations::RunMigrationsError;
use std::io::stdout;

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

impl NewRepository {
    pub fn new(name: String, path: String) -> NewRepository {
        NewRepository { name, path }
    }
}

pub fn run_pending_migrations() -> Result<(), RunMigrationsError> {
    let connection = establish_connection();
    embedded_migrations::run_with_output(&connection, &mut stdout())
}

pub fn create_repository<'a>(new_repository: &'a NewRepository) -> Result<Repository, Error> {
    use crate::schema::repositories::dsl::*;
    let connection = establish_connection();

    diesel::insert_into(repositories)
        .values(new_repository)
        .execute(&connection)?;

    // Get the last inserted repository
    // get_result isn't implemented with SQLite
    repositories.find(&new_repository.name).first(&connection)
}

pub fn get_repositories() -> Result<Vec<Repository>, Error> {
    use crate::schema::repositories::dsl::*;
    let connection = establish_connection();

    repositories.load::<Repository>(&connection)
}

pub fn get_repository(repo_name: String) -> Result<Repository, Error> {
    use crate::schema::repositories::dsl::*;
    let connection = establish_connection();

    repositories.find(&repo_name).first(&connection)
}

pub fn remove_repository(repo_name: String) -> Result<usize, Error> {
    use crate::schema::repositories::dsl::*;
    let connection = establish_connection();

    diesel::delete(repositories)
        .filter(name.eq(repo_name))
        .execute(&connection)
}
