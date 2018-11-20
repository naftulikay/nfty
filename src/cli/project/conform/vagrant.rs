#[derive(Debug, StructOpt)]
pub struct Ansible {
    #[structopt(long = "docker")]
    pub docker: bool,
    #[structopt(long = "go-version")]
    pub go_version: Option<String>,
    #[structopt(long = "go-package")]
    pub go_package: Option<String>,
    #[structopt(long = "java")]
    pub java: bool,
    #[structopt(long = "python-version")]
    pub python_version: Option<String>,
    #[structopt(long = "ruby-version")]
    pub ruby_version: Option<String>,
    #[structopt(long = "rust")]
    pub rust: bool,
    #[structopt(long = "node-version")]
    pub node_version: Option<String>,
}
