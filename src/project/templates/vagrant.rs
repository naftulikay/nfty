#[cfg(test)]
mod tests;

use askama::Template;

use std::io::Write;

#[derive(Default, Template, WritableTemplate)]
#[template(path = "vagrant/Vagrantfile.j2")]
pub struct Vagrantfile {}

impl Vagrantfile {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Template, WritableTemplate)]
#[template(path = "vagrant/playbook.yml.j2")]
pub struct VagrantPlaybook {
    pub include_docker: bool,
    pub go_version: Option<String>,
    pub go_package: Option<String>,
    pub java: bool,
    pub python_version: Option<String>,
    pub ruby_version: Option<String>,
    pub include_rust: bool,
    pub node_version: Option<String>,
}

impl VagrantPlaybook {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn docker(mut self) -> Self {
        self.include_docker = true;
        self
    }

    pub fn go(mut self, package_name: &str, version: &str) -> Self {
        self.go_package = Some(package_name.into());
        self.go_version = Some(version.into());
        self
    }

    pub fn java(mut self) -> Self {
        self.java = true;
        self
    }

    pub fn node(mut self, version: &str) -> Self {
        self.node_version = Some(version.into());
        self
    }

    pub fn python(mut self, version: &str) -> Self {
        self.python_version = Some(version.into());
        self
    }

    pub fn ruby(mut self, version: &str) -> Self {
        self.ruby_version = Some(version.into());
        self
    }

    pub fn rust(mut self) -> Self {
        self.include_rust = true;
        self
    }

    #[inline]
    pub fn has_docker(&self) -> bool {
        self.include_docker
    }

    #[inline]
    pub fn has_go(&self) -> bool {
        self.go_version.is_some()
    }

    #[inline]
    pub fn has_java(&self) -> bool {
        self.java
    }

    #[inline]
    pub fn has_node(&self) -> bool {
        self.node_version.is_some()
    }

    #[inline]
    pub fn has_python(&self) -> bool {
        self.python_version.is_some()
    }

    #[inline]
    pub fn has_ruby(&self) -> bool {
        self.ruby_version.is_some()
    }

    #[inline]
    pub fn has_rust(&self) -> bool {
        self.include_rust
    }
}
