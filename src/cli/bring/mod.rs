use rayon::prelude::*;

use api::project::Project;

use pbr::MultiBar;

use std::error::Error;
use std::io::stderr;
use std::process;
use std::thread;

use util::Git;

#[derive(Debug,StructOpt)]
pub struct Bring {
    /// A list of repositories to fetch.
    pub repositories: Vec<String>,
}

impl Bring {

    pub fn execute(&self) {
        let mut multibar = MultiBar::on(stderr());

        // create an array of bars
        let bars = self.repositories.iter().map(|_| multibar.create_bar(100)).collect::<Vec<_>>();

        let finish = thread::spawn(move || multibar.listen());

        self.repositories.par_iter().zip(bars).for_each(|(repository, mut bar)| {
            let project = match Project::from(&repository) {
                Ok(project) => project,
                Err(_)      => {
                    error!("Unable to parse repository URL: {}", repository);
                    process::exit(1);
                }
            };

            // set prefix
            bar.show_counter = false;
            bar.show_speed = false;
            bar.message(&format!("{}/{}: ", project.owner(), project.repository()));

            // clone the repository; this is idempotent - will only clone if repo doesn't exist
            info!("Cloning remote repository {}", &project);

            // force a draw of the progress bar
            bar.tick();

            let clone_result = project.clone(|progress| {
                bar.set(Git::clone_progress(&progress));
                true
            });

            // always finish
            bar.finish_print(&format!("{}/{}: {}", project.owner(), project.repository(), match clone_result {
                Ok(_)  => "done",
                Err(_) => "failed",
            }));

            // deal with errors or success
            match clone_result {
                Ok(_) => debug!("Successfully cloned repository."),
                Err(e) => {
                    error!("Unable to clone repository {}: {}", &project.url(), e.description());
                    process::exit(1);
                }
            };

            // always install hooks, regardless of whether we cloned or not
            match project.configure() {
                Ok(_)  => debug!("Installed hooks successfully."),
                Err(e) => error!("Failed to install Git hooks: {}", e),
            };
        });

        finish.join().unwrap_or(());
    }
}
