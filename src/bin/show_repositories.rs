extern crate grm;
extern crate diesel;
use self::grm::db::*;
use self::grm::models::*;
use self::diesel::prelude::*;

fn main() {
    use grm::schema::repositories::dsl::*;

    let connection = establish_connection();
    let results = repositories
        .load::<Repository>(&connection)
        .expect("Error loading repositories");

    println!("Displaying {} repositories", results.len());

    for repo in results {
        println!("{}", repo.name);
        println!("----------\n");
        println!("{}", repo.path);
    }

    //let args: Vec<String> = env::args().collect();
    //let project_name = args[1].clone();
    //println!("{}", project_name);
}
