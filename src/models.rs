use schema::repositories;

#[derive(Queryable)]
pub struct Repository {
    pub id: i32,
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