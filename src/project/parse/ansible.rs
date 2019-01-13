#[cfg(test)]
mod tests;

use std::io;

use serde_derive::Deserialize;
use serde_yaml;

#[derive(Debug, Deserialize)]
pub struct RoleMetadata {
    pub galaxy_info: GalaxyInfo,
    pub dependencies: Vec<Role>,
}

impl RoleMetadata {
    pub fn load<T>(reader: T) -> serde_yaml::Result<Self> where T: io::Read {
        Ok(serde_yaml::from_reader(reader)?)
    }

    pub fn deps(&self) -> Vec<String> {
        let mut result = Vec::new();

        for dependency in &self.dependencies {
            match dependency {
                Role::Simple(s) => result.push(s.clone()),
                Role::Complex(r) => result.push(r.role.clone()),
            }
        }

        result
    }
}

#[derive(Debug, Deserialize)]
pub struct GalaxyInfo {
    pub name: String,
    pub author: String,
    pub src: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Role {
    Simple(String),
    Complex(ComplexRole),
}

#[derive(Debug, Deserialize)]
pub struct ComplexRole {
    pub role: String,
    pub src: Option<String>,
    pub name: Option<String>,
}
