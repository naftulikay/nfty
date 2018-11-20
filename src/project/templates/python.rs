use askama::Template;

use std::collections::BTreeSet;
use std::fs;
use std::io::Result;
use std::io::Write;
use std::path::Path;

#[derive(Default, Template, WritableTemplate)]
#[template(path = "python/requirements.txt.j2")]
pub struct PythonRequirements {
    pub requirements: BTreeSet<String>,
}

impl PythonRequirements {
    pub fn load(path: &Path) -> Result<Self> {
        let mut result = Self::default();

        // FIXME this does strip out empty lines and comments, but otherwise this would be a lot
        // harder to do.
        fs::read_to_string(path)?
            .lines()
            .map(|l| l.trim())
            .filter(|l| l.len() > 0)
            .filter(|l| !l.starts_with("#"))
            .for_each(|line| {
                result.requirements.insert(line.to_string());
            });

        Ok(result)
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn requirement(mut self, req: String) -> Self {
        self.requirements.insert(req);
        self
    }
}
