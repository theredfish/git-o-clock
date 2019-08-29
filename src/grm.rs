use crate::config::Config;
use crate::db::{self, models::*};
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;
use dunce;
use glob::glob;
use std::path::Path;
use std::process;

const ERROR_OPEN_ISSUE: &str = "If you think this is a bug, feel free to open an issue : https://github.com/theredfish/git-repo-manager/issues/new";

pub struct Grm {
    config: Config,
}

impl Grm {
    pub fn new(config: Config) -> Self {
        // Run pending migrations - used to initialize and update the SQLite database.
        // TODO : prompt a question to know if the user wants to apply the update.
        if let Err(e) = db::run_pending_migrations() {
            eprintln!("Update failed : {}", e);
            eprintln!("{}", ERROR_OPEN_ISSUE);
            process::exit(1);
        };

        Grm { config }
    }

    pub fn add(self, path: String) {
        let found_repos = self.search(path);

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

    pub fn goto(self, repo_name: String) {
        match get_repository(repo_name) {
            Ok(repo) => println!("{}", Path::new(&repo.path).display()),
            Err(e) => eprintln!("Cannot retrieve the repository : {}", e),
        }
    }

    pub fn list(self) {
        match get_repositories() {
            Ok(repos) => {
                if !repos.is_empty() {
                    for repo in repos {
                        println!("{}", repo.name)
                    }
                } else {
                    println!("No repository found - try to add a path or to run `grm refresh`.");
                }
            }
            Err(e) => eprintln!("Cannot list repositories : {}", e),
        }
    }

    pub fn rm(self, repo_name: String) {
        match remove_repository(repo_name) {
            Ok(count) => println!("{} project(s) removed", count),
            Err(e) => eprintln!("Cannot remove the repository : {}", e),
        }
    }

    fn search(self, path: String) -> Vec<NewRepository> {
        // let glob_path = path + &self.config.depth_to_glob();
        let mut repositories: Vec<NewRepository> = Vec::new();
        let mut max_glob = path + "/";

        println!("Searching for git repositories (it may take a few seconds).");

        fn path_buf_to_repo(path_buf: std::path::PathBuf) -> NewRepository {
            let absolute_path = dunce::canonicalize(path_buf).unwrap();
            let parent = absolute_path.parent().unwrap();
            let name = parent.iter().last().unwrap().to_str().unwrap();
            let path = parent.to_str().unwrap();

            NewRepository::new(String::from(name), String::from(path))
        }

        // For each depth, check if a .git exists
        for d in 0..self.config.max_depth_search {
            // we check in sub folders
            if d > 0 {
                max_glob += "*/";
            }

            println!("depth : {}; glob : {:?}", d, max_glob);

            // For each glob depth
            let mut new_repositories: Vec<NewRepository> = glob(&(max_glob.to_owned() + ".git"))
                .unwrap()
                .filter_map(Result::ok)
                .map(path_buf_to_repo)
                .collect();

            repositories.append(&mut new_repositories);
        }

        repositories
    }
}
