pub mod parse;
pub mod templates;

mod hooks;

#[cfg(test)]
mod test;

use dirs::home_dir;

use lazy_static::lazy_static;

use regex::Regex;

use std::fmt;
use std::io;
use std::path::PathBuf;

use git2::build::RepoBuilder;
use git2::Cred;
use git2::FetchOptions;
use git2::Progress;
use git2::RemoteCallbacks;
use git2::Repository;

lazy_static! {
    // match https protocol git clone thingamajigs
    static ref HTTPS_PROVIDER: Regex = Regex::new(r#"(?x)
        \b(?P<protocol>https)://(?P<host>[^/\s]+)/(?P<owner>[^/\s]+)/(?P<repository>[^/\s]+)\b
    "#).unwrap();

    // match ssh and short form
    static ref SSH_PROVIDER: Regex = Regex::new(r#"(?x)
        \b(?:(?:(?P<user>[^@\s/]+)@)?(?P<host>[^:\s/]+)[:/])?(?P<owner>[^/\s]+)/(?P<repository>[^/\s]+)\b
    "#).unwrap();

    pub static ref PROJECT_ROOT: PathBuf = home_dir().expect("unable to get home dir").join("devel").join("src");
}

static ERR_PROJECT_NAME: &'static str =
    r#"Project name must be formatted in "organization/repository" format."#;

pub static DEFAULT_USER: &'static str = "git";
pub static DEFAULT_HOST: &'static str = "github.com";

pub struct Project {
    host: String,
    owner: String,
    protocol: Protocol,
    _raw: String,
    repository: String,
    user: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Protocol {
    Https,
    Ssh,
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.protocol {
            Protocol::Https => write!(
                f,
                "https://{}/{}/{}",
                self.host, self.owner, self.repository
            ),
            Protocol::Ssh => write!(
                f,
                "{}@{}:{}/{}",
                self.user(),
                self.host,
                self.owner,
                self.repository
            ),
        }
    }
}

impl Project {
    /// Create a new project object from a string.
    ///
    /// Acceptable formats for SSH transport:
    ///   - naftulikay/gro
    ///   - naftulikay/gro.git
    ///   - github.com:naftulikay/gro
    ///   - github.com:naftulikay/gro.git
    ///   - git@github.com:naftulikay/gro
    ///   - git@github.com:naftulikay/gro.git
    ///
    /// Acceptable formats for TLS transport:
    ///   - https://github.com/naftulikay/gro
    ///   - https://github.com/naftulikay/gro.git
    ///
    pub fn from(value: &str) -> Result<Self, io::Error> {
        let value = if value.ends_with(".git") {
            &value[0..value.len() - 4]
        } else {
            value
        };

        if HTTPS_PROVIDER.is_match(value) {
            // anonymous HTTPS urls
            let captures = HTTPS_PROVIDER.captures(value).unwrap();

            Ok(Project {
                host: captures
                    .name("host")
                    .map_or(DEFAULT_HOST, |m| m.as_str())
                    .to_string(),
                owner: captures.name("owner").unwrap().as_str().to_string(),
                protocol: Protocol::Https,
                _raw: value.to_string(),
                repository: captures.name("repository").unwrap().as_str().to_string(),
                user: None,
            })
        } else if SSH_PROVIDER.is_match(value) {
            // default/SSH urls
            let captures = SSH_PROVIDER.captures(value).unwrap();

            Ok(Project {
                host: captures
                    .name("host")
                    .map_or(DEFAULT_HOST, |m| m.as_str())
                    .to_string(),
                owner: captures.name("owner").unwrap().as_str().to_string(),
                protocol: Protocol::Ssh,
                _raw: value.to_string(),
                repository: captures.name("repository").unwrap().as_str().to_string(),
                user: Some(
                    captures
                        .name("user")
                        .map_or(DEFAULT_USER, |m| m.as_str())
                        .to_string(),
                ),
            })
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, ERR_PROJECT_NAME))
        }
    }

    /// Get the protocol of the repository.
    pub fn protocol(&self) -> &Protocol {
        &self.protocol
    }

    /// Get the host name of the repository.
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Get the user by which to connect to the remote repository.
    ///
    /// If protocol is `Protocol::Https`, this value is irrelevant.
    pub fn user(&self) -> &str {
        self.user.as_ref().map_or(DEFAULT_USER, |m| m.as_str())
    }

    /// Owner of the repository.
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// The name of the repository.
    pub fn repository(&self) -> &str {
        &self.repository
    }

    /// Get the URL by which to clone the given project.
    pub fn url(&self) -> String {
        match self.protocol {
            Protocol::Https => format!("https://{}/{}/{}", self.host, self.owner, self.repository),
            Protocol::Ssh => format!(
                "{}@{}:{}/{}",
                self.user(),
                self.host,
                self.owner,
                self.repository
            ),
        }
    }

    /// Get the directory of a given project.
    pub fn dir(&self) -> PathBuf {
        PROJECT_ROOT
            .join(&self.host)
            .join(&self.owner)
            .join(&self.repository)
    }

    /// Determine whether a project locally exists.
    pub fn is_local(&self) -> bool {
        let project_dir = self.dir();

        return project_dir.exists() && project_dir.is_dir();
    }

    /// Clone the repository.
    pub fn clone<F>(&self, callback: F) -> io::Result<Repository>
    where
        F: FnMut(Progress) -> bool,
    {
        if let Ok(repo) = Repository::open(&self.dir()) {
            // if the repository already exists, don't clone it, just return this.
            return Ok(repo);
        }

        let mut callbacks = RemoteCallbacks::new();

        // set credentials
        callbacks.credentials(|_a, _b, _cred_type| Cred::ssh_key_from_agent(&self.user()));

        // set transfer progress
        callbacks.transfer_progress(callback);

        // add them to fetch options
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        // add them to the builder
        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_options);

        match builder.clone(&self.url(), &self.dir()) {
            Ok(r) => Ok(r),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string().trim())),
        }
    }

    /// Configure the repository, install hooks, etc.
    pub fn configure(&self) -> io::Result<()> {
        hooks::install(&self.dir())
    }
}
