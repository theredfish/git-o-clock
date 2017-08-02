extern crate grm;
extern crate rusqlite;

use rusqlite::Connection;
use std::env;
use std::process;
use std::path::Path;

use grm::cmd_parser::Config;
use grm::repo_manager::GitRepo;
use grm::db;

fn main() {
    let conn = db::initialize_connection().unwrap_or_else(|err| {
        println!("SQLite database error : {} ", err);
        process::exit(1);
    });

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // TODO check if the path exists before to continue

    /*let path_exists = Path::new(config.path).exists();
    match false {

    }*/

    run(conn, config);
}

fn run(conn: Connection, config: Config) {
    let pattern = String::from("/**/*.git");
    let git_repo = GitRepo::new(conn, pattern);

    match config.query.as_ref() {
        "add" => git_repo.add(String::from(config.path)),
        _ => {
            println!("{} : command not found", config.query);
            process::exit(1);
        },
    }
}