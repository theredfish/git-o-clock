embed_migrations!("migrations");

use db::establish_connection;
use std::io::stdout;
use super::schema::repositories;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Queryable)]
#[derive(Debug)]
pub struct Repository {
    pub name: String,
    pub path: String
}

#[derive(Insertable)]
#[derive(Debug)]
#[table_name="repositories"]
pub struct NewRepository {
    pub name: String,
    pub path: String
}

impl NewRepository {
    pub fn new(name: String, path: String) -> NewRepository {
        NewRepository {
            name,
            path
        }
    }
}

pub fn run_pending_migrations() {
    let connection = establish_connection();
    embedded_migrations::run_with_output(&connection, &mut stdout());
}

pub fn create_repository<'a>(new_repository: &'a NewRepository) -> Result<Repository, Error> {
    use schema::repositories::dsl::*;
    let connection = establish_connection();

    diesel::insert_into(repositories)
        .values(new_repository)
        .execute(&connection)?;

    // Get the last inserted repository
    // get_result isn't implemented with SQLite
    repositories
        .find(&new_repository.name)
        .first(&connection)
}

pub fn get_repositories() -> Result<Vec<Repository>, Error> {
    use schema::repositories::dsl::*;
    let connection = establish_connection();

    repositories.load::<Repository>(&connection)

}

pub fn get_repository(repo_name: String) -> Result<Repository, Error> {
    use schema::repositories::dsl::*;
    let connection  = establish_connection();

    repositories.find(&repo_name).first(&connection)
}