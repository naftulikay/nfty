use rayon::prelude::*;

use cli::project::GitHubProject;
use cli::data::project::bring::Bring;

use git2::Cred;
use git2::FetchOptions;
use git2::RemoteCallbacks;
use git2::build::RepoBuilder;

use std::env;

pub fn execute(bring: Bring) {
    let projects_root = env::home_dir().expect("unable to get home dir").join("Documents/Development");

    bring.repositories.par_iter().for_each(|repository| {
        println!("repository: {}", repository);
        let project = match GitHubProject::from(repository) {
            Ok(project) => project,
            Err(_)      => {
                error!("Unable to clone repository: {}", repository);
                return;
            }
        };

        let url = format!("git@github.com:{}/{}.git", project.owner(), project.repository());
        let path = projects_root.join(project.owner()).join(project.repository());

        if !path.is_dir() {
            let mut callbacks = RemoteCallbacks::new();

            // set credentials
            callbacks.credentials(|_a, _b, _cred_type| {
               Cred::ssh_key_from_agent(&"git")
            });

            // add them to fetch options
            let mut fetch_options = FetchOptions::new();
            fetch_options.remote_callbacks(callbacks);

            // add them to the builder
            let mut builder = RepoBuilder::new();
            builder.fetch_options(fetch_options);

            match builder.clone(&url, &path) {
                Ok(_)  => debug!("Successfully cloned {}", repository),
                Err(_) => {
                    error!("Unable to clone {}", repository);
                    return
                }
            };
        }
    });
 }
