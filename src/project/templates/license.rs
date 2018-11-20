use askama::Template;

use chrono::Datelike;
use chrono::NaiveDate;
use chrono::Utc;

use crate::project::templates::WritableTemplate;

use log::debug;
use log::info;

use std::default::Default;
use std::fs;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::Result;
use std::path::Path;

#[derive(Template,WritableTemplate)]
#[template(path = "licenses/APACHE.j2")]
pub struct ApacheLicense<'a> {
    pub author: &'a str,
    pub date: NaiveDate,
}

impl ApacheLicense<'_> {
    pub fn write(&self, dest: &Path) -> Result<()> {
        let mut file = BufWriter::new(fs::File::create(dest)?);
        file.write(self.render().unwrap().as_bytes())?;

        Ok(())
    }
}

impl Default for ApacheLicense<'_> {
    fn default() -> Self {
        ApacheLicense {
            author: "Naftuli Kay".into(),
            date: Utc::now().date().naive_utc(),
        }
    }
}

#[derive(Template,WritableTemplate)]
#[template(path = "licenses/MIT.j2")]
pub struct MitLicense<'a> {
    pub author: &'a str,
    pub date: NaiveDate,
}

impl Default for MitLicense<'_> {
    fn default() -> Self {
        MitLicense {
            author: "Naftuli Kay".into(),
            date: Utc::now().date().naive_utc(),
        }
    }
}

/// Apply licensing in the given project directory.
pub fn apply(project_dir: &Path, private: bool) -> Result<()> {
    if private {
        apply_private(project_dir)
    } else {
        apply_public(project_dir)
    }
}

/// Apply public licensing in the given project directory.
pub fn apply_public(project_dir: &Path) -> Result<()> {
    info!(
        "Applying public Apache and MIT licensing to the project at {}",
        project_dir.display()
    );

    // if LICENSE exists, attempt to remove it in favor of dual licensing
    let old_license = &project_dir.join("LICENSE");

    if old_license.exists() {
        debug!("Removing old LICENSE");
        fs::remove_file(&old_license)?;
    }

    // render apache
    ApacheLicense::default().write(&project_dir.join("LICENSE-APACHE"))?;

    // render mit
    MitLicense::default().write(&project_dir.join("LICENSE-MIT"))?;

    Ok(())
}

/// Apply private licensing in the given projec directory.
pub fn apply_private(project_dir: &Path) -> Result<()> {
    info!(
        "Applying private licensing to the project at {}",
        project_dir.display()
    );

    Ok(())
}
