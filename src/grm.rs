use crate::db::models::*;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;
use dunce;
use glob::glob;
use std::path::Path;

pub fn add(path: &str, term: &str) {
    let found_repos = search(with_pattern, path, term);

    for repo in found_repos {
        match create_repository(&repo) {
            Ok(inserted_repo) => println!("-> {}", inserted_repo.name),
            Err(DatabaseError(UniqueViolation, _)) => {
                eprintln!("{} : name already exists", repo.name)
            }
            Err(e) => eprintln!("Cannot create the repository {} : {}", repo.name, e),
        };
    }
}

pub fn list() {
    match get_repositories() {
        Ok(repos) => {
            if !repos.is_empty() {
                for repo in repos {
                    println!("{}", repo.name)
                }
            } else {
                println!("No repository found. Tell me to add more and I will execute.");
            }
        }
        Err(e) => eprintln!("Cannot list repositories : {}", e),
    }
}

pub fn goto(repo_name: String) {
    match get_repository(repo_name) {
        Ok(repo) => println!("{}", Path::new(&repo.path).display()),
        Err(e) => eprintln!("Cannot retrieve the repository : {}", e),
    }
}

pub fn rm(repo_name: String) {
    match remove_repository(repo_name) {
        Ok(count) => println!("{:?} project(s) removed", count),
        Err(e) => eprintln!("Cannot remove the repository : {}", e),
    }
}

fn search<F: Fn(&str, &str) -> String>(f: F, path: &str, pattern: &str) -> Vec<NewRepository> {
    let path_pattern = f(path, pattern);
    let mut repositories: Vec<NewRepository> = Vec::new();

    println!("Please wait, I'm scanning your projects...");

    for path in glob(&path_pattern).unwrap().filter_map(Result::ok) {
        let absolute_path = dunce::canonicalize(path).unwrap();
        let parent = absolute_path.parent().unwrap();
        let name = parent.iter().last().unwrap().to_str().unwrap();
        let path = parent.to_str().unwrap();

        let repo = NewRepository::new(String::from(name), String::from(path));
        repositories.push(repo);
    }

    repositories
}

fn with_pattern(path: &str, pattern: &str) -> String {
    path.to_string() + pattern
}
