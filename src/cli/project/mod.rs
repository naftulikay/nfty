pub mod bring;
pub mod engage;

use cli::data::project::Project;
use cli::data::project::ProjectCommand;

use regex::Regex;

use std::env::home_dir;
use std::fmt;
use std::io;
use std::path::Path;
use std::path::PathBuf;

lazy_static! {
    static ref PROJECT: Regex = Regex::new(r#"(?:(?P<owner>[^/]+)/)?(?P<repository>[^/]+)"#).unwrap();
}

pub struct GitHubProject {
    name: String,
}

impl fmt::Display for GitHubProject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.owner(), self.repository())
    }
}

impl GitHubProject {

    pub fn from(name: &str) -> Result<Self, io::Error> {
        if PROJECT.is_match(name) {
            Ok(GitHubProject { name: name.to_string() })
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, r#"Project name must be formatted in "organization/repository" format."#))
        }
    }

    pub fn owner(&self) -> &str {
        for captures in PROJECT.captures_iter(&self.name) {
           return captures.name("owner").map(|m| m.as_str()).unwrap_or(&"naftulikay");
        }

        unreachable!("the regular expression matched at creation time");
    }

    pub fn repository(&self) -> &str {
        for captures in PROJECT.captures_iter(&self.name) {
            return captures.name("repository").map(|m| m.as_str()).unwrap();
        }

        unreachable!("the regular expression matched at creation time");
    }
}

pub fn execute(project: Project) {
    match project.command {
        ProjectCommand::Bring(e) => bring::execute(e),
        ProjectCommand::Engage(e) => engage::execute(e),
    }
}

/// Get the directory of a given project.
pub fn get_project_dir(project: &str) -> PathBuf {
    home_dir().expect("unable to get home directory")
        .join(Path::new("Documents/Development"))
        .join(Path::new(project))

}

/// Determine whether a project locally exists.
pub fn project_exists(project: &str) -> bool {
    let project_dir = get_project_dir(project);

    return project_dir.exists() && project_dir.is_dir()
}
