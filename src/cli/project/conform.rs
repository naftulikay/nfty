mod ansible;

use crate::project::templates::license;

use git2::Repository;

use log::error;

use std::path::PathBuf;
use std::process::exit;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Conform {
    /// Project should be assumed to be private rather than the default, which is public.
    #[structopt(short = "P", long = "private")]
    pub private: bool,
    #[structopt(subcommand)]
    pub template: ProjectTemplate,
}

#[derive(Debug, StructOpt)]
pub enum ProjectTemplate {
    /// Conform and render an Ansible role project.
    #[structopt(name = "ansible")]
    Ansible(ansible::Ansible),
}

impl Conform {

    pub fn execute(&self) {
        // discover the root directory of the current git repository
        let repo: Repository = Repository::open_from_env().map_err(|e| {
            error!("Unable to open a Git repository from the current directory: {}", e);
            exit(1)
        }).unwrap();

        let root: PathBuf = repo.workdir().unwrap_or_else(|| {
            error!("Unable to find the root directory of this Git repository");
            exit(1)
        }).into();

        // apply licensing
        license::apply(&root, self.private).map_err(|e| {
            error!("Unable to apply licensing: {}", e);
            exit(1)
        }).ok();

        // do the actual thing
        match self.template {
            ProjectTemplate::Ansible(ref t) => t.execute(&repo, &root),
        }
    }
}
