use glob::glob;
use models::*;
use dunce;
use diesel::result::Error::DatabaseError;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use std::path::Path;

pub fn update_grm() {
    if let Err(e) = run_pending_migrations() {
        eprintln!("GRM update failed : {}", e);
        eprintln!("Please open an issue with your output : https://github.com/theredfish/git-repo-manager/issues/new");
    }
}

pub fn add(path: String, term: String) {
    let found_repos = search(with_pattern, path, term);

   for repo in found_repos {
       match create_repository(&repo) {
           Ok(inserted_repo) => println!("-> {}", inserted_repo.name),
           Err(DatabaseError(UniqueViolation, _)) => eprintln!("{} : name already exists", repo.name),
           Err(e) => eprintln!("Cannot create the repository {} : {}", repo.name, e)
       };
   }
}

pub fn list() {
    match get_repositories() {
        Ok(repos) => {
            if repos.len() > 0 {
                for repo in repos {
                    println!("{}", repo.name)
                }
            } else {
                println!("No repository found. Tell me to add more and I will execute.");
            }
        }
        Err(e) => eprintln!("Cannot list repositories : {}", e)
    }
}

pub fn goto(repo_name: String) {
    match get_repository(repo_name) {
        Ok(repo) => println!("{}", Path::new(&repo.path).display()),
        Err(e) => eprintln!("Cannot retrieve the repository : {}", e)
    }
}

pub fn rm(repo_name: String) {
    match remove_repository(repo_name) {
        Ok(count) => println!("{:?} project(s) removed", count),
        Err(e) => eprintln!("Cannot remove the repository : {}", e)
    }
}

fn search<F: Fn(String, String) -> String>(f: F, path: String, pattern: String) -> Vec<NewRepository> {
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

fn with_pattern(path: String, pattern: String) -> String {
    let path_pattern = path.to_string() + &pattern.to_string();
    String::from(path_pattern)
}