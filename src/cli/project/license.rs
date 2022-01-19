use anyhow::{anyhow, Result};
use askama::Template;
use chrono::{Datelike, NaiveDate, Utc};
use nfty_derive::WritableTemplate;
use std::env;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

use crate::project::templates::WritableTemplate;

#[derive(Debug, StructOpt)]
pub struct License {
    /// The type of license to generate. Options can be oss, mit, apache2, and private. 'oss' is a
    /// special value, it implies both MIT and Apache2.
    #[structopt(short = "t", long = "type", default_value)]
    pub license_type: LicenseType,
    /// Disable deletion other licenses before license generation. By default, all files matching
    /// `LICENSE*` will be removed before rendering new licenses.
    #[structopt(long = "no-clean")]
    pub no_clean: bool,
    /// The root directory of the project to manage licenses for. Defaults to the current directory.
    #[structopt(long = "dir")]
    pub project_dir: Option<PathBuf>,
    #[structopt(short = "a", long = "author", default_value = "Naftuli Kay")]
    /// The author/copyright holder for the license(s).
    pub license_holder: String,
}

#[derive(Debug)]
pub enum LicenseType {
    Oss,
    Mit,
    Apache2,
    Private,
}

impl Default for LicenseType {
    fn default() -> Self {
        Self::Oss
    }
}

impl Display for LicenseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Oss => "oss",
                Self::Mit => "mit",
                Self::Apache2 => "apache2",
                Self::Private => "private",
            }
        )
    }
}

impl FromStr for LicenseType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_ref() {
            "oss" | "default" => Ok(Self::Oss),
            "mit" => Ok(Self::Mit),
            "apache" | "apache2" => Ok(Self::Apache2),
            "private" => Ok(Self::Private),
            s => Err(anyhow!("Unknown license type: {}", s)),
        }
    }
}

impl License {
    pub fn execute(&self) {
        if !self.no_clean {
            if let Err(e) = self.clean() {
                log::error!("Unable to clean up existing license files: {}", e);
            }
        }

        let mut licenses = Vec::with_capacity(2);

        match &self.license_type {
            LicenseType::Oss => {
                licenses.push(&LicenseType::Mit);
                licenses.push(&LicenseType::Apache2);
            }
            lt => licenses.push(lt),
        }

        let multi_license = licenses.len() > 1;

        let cwd = env::current_dir();
        let dir = self.project_dir.as_ref().unwrap_or(
            cwd.as_ref()
                .expect("could not get current working directory"),
        );

        for license_type in licenses {
            match license_type {
                LicenseType::Apache2 => {
                    let template = ApacheLicense {
                        author: self.license_holder.as_str(),
                        date: Utc::today().naive_local(),
                    };

                    let filename = if multi_license {
                        "LICENSE-APACHE"
                    } else {
                        "LICENSE"
                    };

                    log::info!(
                        "Rendering Apache Software License, Version 2.0 to {}...",
                        filename
                    );
                    template
                        .write(&dir.join(filename))
                        .expect("unable to render license");
                }
                LicenseType::Mit => {
                    let template = MitLicense {
                        author: self.license_holder.as_str(),
                        date: Utc::today().naive_local(),
                    };

                    let filename = if multi_license {
                        "LICENSE-MIT"
                    } else {
                        "LICENSE"
                    };

                    log::info!("Rendering MIT License to {}...", filename);
                    template
                        .write(&dir.join(filename))
                        .expect("unable to render license");
                }
                LicenseType::Private => {
                    let template = PrivateLicense {
                        author: self.license_holder.as_str(),
                        date: Utc::today().naive_local(),
                    };

                    log::info!("Rendering private license...");
                    template
                        .write(&dir.join("LICENSE"))
                        .expect("unable to render license");
                }
                LicenseType::Oss => unreachable!(),
            }
        }
    }

    fn clean(&self) -> Result<()> {
        log::info!("Removing all existing licenses before generating new ones.");

        let cwd = env::current_dir();
        let dir = self.project_dir.as_ref().unwrap_or(
            cwd.as_ref()
                .map_err(|e| anyhow!("unable to get current directory: {}", e))?,
        );

        // find all files in the given directory which start with `LICENSE` and remove them.
        for license_file in dir
            .read_dir()
            .map_err(|e| anyhow!("unable to read directory {}: {}", dir.display(), e))?
            .into_iter()
            .filter_map(|r| r.ok())
            .filter(|d| d.file_type().unwrap().is_file())
            .filter(|d| {
                d.file_name()
                    .to_string_lossy()
                    .as_ref()
                    .starts_with("LICENSE")
            })
            .map(|d| d.path())
        {
            log::info!("Removing old license file {}", license_file.display());
            fs::remove_file(&license_file).map_err(|e| {
                anyhow!(
                    "unable to remove license file {}: {}",
                    license_file.display(),
                    e
                )
            })?;
        }

        Ok(())
    }
}

#[derive(Template, WritableTemplate)]
#[template(path = "licenses/APACHE.j2")]
pub struct ApacheLicense<'a> {
    pub author: &'a str,
    pub date: NaiveDate,
}

#[derive(Template, WritableTemplate)]
#[template(path = "licenses/MIT.j2")]
pub struct MitLicense<'a> {
    pub author: &'a str,
    pub date: NaiveDate,
}

#[derive(Template, WritableTemplate)]
#[template(path = "licenses/PRIVATE.j2")]
pub struct PrivateLicense<'a> {
    pub author: &'a str,
    pub date: NaiveDate,
}
