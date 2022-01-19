mod bring;
mod conform;
mod engage;
mod license;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Project {
    /// Bring down a remote project.
    #[structopt(name = "bring")]
    Bring(bring::Bring),
    /// Conform projects to a given project template.
    #[structopt(name = "conform")]
    Conform(conform::Conform),
    /// Manage software projects.
    #[structopt(name = "engage")]
    Engage(engage::Engage),
    /// Generate software licenses for a project.
    #[structopt(name = "license")]
    License(license::License),
}

impl Project {
    pub fn execute(&self) {
        match self {
            Project::Bring(ref c) => c.execute(),
            Project::Conform(ref c) => c.execute(),
            Project::Engage(ref c) => c.execute(),
            Project::License(ref c) => c.execute(),
        }
    }
}
