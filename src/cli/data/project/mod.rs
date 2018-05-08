pub mod bring;
pub mod engage;

#[derive(Debug,StructOpt)]
pub struct Project {
    #[structopt(subcommand)]
    pub command: ProjectCommand,
}

#[derive(Debug,StructOpt)]
pub enum ProjectCommand {
    /// Fetch and setup a project locally from GitHub.
    #[structopt(name="bring")]
    Bring(bring::Bring),
    /// Start work in a given project.
    #[structopt(name="engage")]
    Engage(engage::Engage),
}
