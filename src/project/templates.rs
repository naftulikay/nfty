pub mod ansible;
pub mod git;
pub mod license;
pub mod python;
pub mod vagrant;

use askama::Template;

use std::path::Path;
use std::io::Result;

pub trait WritableTemplate: Template {
    fn write(&self, path: &Path) -> Result<()>;
}
