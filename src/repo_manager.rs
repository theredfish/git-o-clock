use glob::glob;
use models::*;
use std::fs;
use std::result::Result;

#[derive(Debug)]
pub struct GitRepo {
    pattern: String
}

impl GitRepo {
    pub fn new(pattern: String) -> GitRepo {
        GitRepo {
            pattern: String::from(pattern)
        }
    }

    pub fn add(&self, path: String) {
        let pattern = path.to_string() + &self.pattern.to_string();
        let repositories = self.search(pattern);

        println!("\nAdding your repositories...\n");

        for new_repo in repositories {
            let repo = create_repository(&new_repo);

            // TODO : Handle unique contraint error and display it
            // in a more user friendly way
            // See https://github.com/diesel-rs/diesel/blob/73778af0827cb066bf1efdb348f380da14c916d4/diesel_tests/tests/errors.rs
            match repo {
                Ok(repo) => println!("-> {}", repo.name),
                Err(err) => {
                    println!("Can't save a repository {}", err);
                    continue;
                }
            }
        }
    }

    fn search(&self, pattern: String) -> Vec<NewRepository> {
        let mut repositories: Vec<NewRepository> = Vec::new();

        println!("I'm searching into these repositories :\n");

        for path in glob(&pattern).unwrap().filter_map(Result::ok) {
            let absolute_path = match fs::canonicalize(path) {
                Ok(canonicalized_path) => canonicalized_path,
                Err(_) => continue
            };

            let parent = absolute_path.parent().unwrap();
            let name = parent.iter().last().unwrap().to_str().unwrap();
            let parent_path: Vec<&str> = parent.to_str().unwrap().split(".git").collect();
            let path = parent_path[0];
            println!("{:?}", path);

            let repo = NewRepository::new(
                String::from(name),
                String::from(path)
            );

            repositories.push(repo);
        }

        repositories
    }
}
