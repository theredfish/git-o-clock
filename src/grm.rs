use crate::config::Config;
use crate::db::{models::*, SqliteDatabase};
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;
use dunce;
use globwalk;
use std::path::Path;
use std::process;

const ERROR_OPEN_ISSUE: &str = "If you think this is a bug, feel free to open an issue : https://github.com/theredfish/git-repo-manager/issues/new";

pub struct Grm {
    config: Config,
}

impl Grm {
    pub fn new(config: Config) -> Self {
        let db = SqliteDatabase::new();

        // Run pending migrations - used to initialize and update the SQLite database.
        // TODO : prompt a question to know if the user wants to apply the update.
        if let Err(e) = db.run_pending_migrations() {
            eprintln!("GRM update failed : {}", e);
            eprintln!("{}", ERROR_OPEN_ISSUE);
            process::exit(1);
        };

        Grm { config }
    }

    pub fn add(self, path: String) {
        let found_repos = self.search(path.as_str());

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

    fn search(self, path: &str) -> Vec<NewRepository> {
        println!("Searching for git repositories (it may take a few seconds).");

        fn dir_entry_to_repo(dir_entry: globwalk::DirEntry) -> NewRepository {
            let absolute_path = dunce::canonicalize(dir_entry.path()).unwrap();
            let parent = absolute_path.parent().unwrap();
            let name = parent.iter().last().unwrap().to_str().unwrap();
            let path = parent.to_str().unwrap();

            NewRepository::new(String::from(name), String::from(path))
        }

        let mut patterns = vec![".git".to_string()];

        if let Some(ignored_folders) = self.config.ignored_folders {
            for folder in ignored_folders {
                let ignored_folder = "!".to_string() + &folder;
                patterns.push(ignored_folder);
            }
        }

        let repositories: Vec<NewRepository> =
            globwalk::GlobWalkerBuilder::from_patterns(path.to_string(), &patterns)
                .max_depth(self.config.max_depth_search)
                .build()
                .unwrap()
                .filter_map(Result::ok)
                .map(dir_entry_to_repo)
                .collect();

        repositories
    }
}
