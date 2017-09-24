use schema::repositories;
use db::*;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Queryable)]
pub struct Repository {
    pub name: String,
    pub path: String
}

#[derive(Insertable)]
#[table_name="repositories"]
pub struct NewRepository{
    pub name: String,
    pub path: String
}

impl NewRepository {
    pub fn new(name: String, path: String) -> NewRepository {
        NewRepository {
            name: name,
            path: path
        }
    }
}

pub fn create_repository<'a>(new_repository: &'a NewRepository) -> Result<Repository, Error> {
    use schema::repositories;

    let connection = establish_connection();

    // continue or return Error
    diesel::insert(new_repository)
        .into(repositories::table)
        .execute(&connection)?;

    // Get the last inserted repository
    // get_result isn't implemented with SQLite
    repositories::table
        .find(&new_repository.name)
        .first(&connection)
}
