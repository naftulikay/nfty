use askama::Template;

use std::collections::BTreeSet;
use std::io::prelude::*;

#[derive(Default, Template, WritableTemplate)]
#[template(path = "ansible/ansible.cfg.j2")]
pub struct AnsibleConfig {
    pub role_paths: Vec<String>,
}

impl AnsibleConfig {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn role_path(mut self, path: String) -> Self {
        self.role_paths.push(path);
        self
    }
}

#[derive(Default, Template, WritableTemplate)]
#[template(path = "ansible/docker-compose.yml.j2")]
pub struct DockerComposeConfig {
    pub dind: bool,
    pub machines: Vec<DockerComposeMachine>,
}

impl DockerComposeConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_dind(&mut self) -> &mut Self {
        self.dind = true;
        self
    }

    pub fn machine(&mut self, machine: DockerComposeMachine) -> &mut Self {
        self.machines.push(machine);
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
pub struct DockerComposeMachine {
    pub name: String,
    pub image: String,
    pub is_privileged: bool,
}

impl DockerComposeMachine {
    pub fn new(name: String, image: String) -> Self {
        DockerComposeMachine {
            name: name,
            image: image,
            is_privileged: false,
        }
    }

    pub fn privileged(mut self) -> Self {
        self.is_privileged = true;
        self
    }
}

#[derive(Default, Template, WritableTemplate)]
#[template(path = "ansible/galaxy-requirements.yml.j2")]
pub struct AnsibleRequirements {
    pub roles: Vec<GalaxyRole>,
}

impl AnsibleRequirements {
    pub fn new() -> Self {
        AnsibleRequirements {
            ..Default::default()
        }
    }

    pub fn role(mut self, role: GalaxyRole) -> Self {
        self.roles.push(role);
        self
    }

    pub fn with_roles(mut self, roles: Vec<GalaxyRole>) -> Self {
        self.roles = roles;
        self
    }
}

pub struct GalaxyRole {
    pub src: String,
    pub name: Option<String>,
    pub version: Option<String>,
}

impl GalaxyRole {
    pub fn new<T>(src: T, name: Option<T>, version: Option<T>) -> Self where T: Into<String> {
        GalaxyRole {
            src: src.into(),
            name: name.map(|v| v.into()),
            version: version.map(|v| v.into()),
        }
    }

    pub fn from_src<T>(src: T) -> Self where T: Into<String> {
        GalaxyRole {
            src: src.into(),
            name: None,
            version: None,
        }
    }
}

#[derive(Default, Template, WritableTemplate)]
#[template(path = "ansible/travis.yml.j2")]
pub struct TravisConfig {}

impl TravisConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default, Template, WritableTemplate)]
#[template(path = "ansible/Makefile.j2")]
pub struct Makefile {}

impl Makefile {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default, Template, WritableTemplate)]
#[template(path = "ansible/tests/gitignore.j2")]
pub struct TestsGitIgnore {}

impl TestsGitIgnore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default, Template, WritableTemplate)]
#[template(path = "ansible/tests/Makefile.j2")]
pub struct TestsMakefile {}

impl TestsMakefile {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default, Template, WritableTemplate)]
#[template(path = "ansible/tests/inventory/hosts.yml.j2")]
pub struct InventoryYaml {
    pub hosts: BTreeSet<String>,
}

impl InventoryYaml {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_host(&mut self, host: String) {
        self.hosts.insert(host);
    }
}
