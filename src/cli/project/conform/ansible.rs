use crate::project::parse;
use crate::project::templates;
use crate::project::templates::WritableTemplate;
use crate::project::templates::ansible::DockerComposeMachine;
use crate::project::templates::ansible::GalaxyRole;

use git2::Repository;

use log::{error, debug, info, warn};

use std::fs;
use std::io;
use std::iter::Iterator;
use std::path::Path;
use std::process::exit;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Ansible {
    /// Disable support for Ubuntu 18.04.
    #[structopt(long = "disable-bionic")]
    pub disable_bionic: bool,
    /// Disable support for CentOS 7.
    #[structopt(long = "disable-centos7")]
    pub disable_centos7: bool,
    /// Disable support for elementary 5.0 Juno.
    #[structopt(long = "disable-juno")]
    pub disable_juno: bool,
    /// Disable support for elementary 0.4 Loki.
    #[structopt(long = "disable-loki")]
    pub disable_loki: bool,
    /// Disable support for Ubuntu 14.04.
    #[structopt(long="disable-trusty")]
    pub disable_trusty: bool,
    /// Disable support for Ubuntu 16.04.
    #[structopt(long = "disable-xenial")]
    pub disable_xenial: bool,
    /// Enable docker-in-docker support for Docker containers.
    #[structopt(long = "enable-dind")]
    pub enable_dind: bool,
    /// The version of Python to use.
    #[structopt(long = "python-version", default_value = "3.6.8")]
    pub python_version: String,
}

impl Ansible {
    pub fn execute(&self, _repo: &Repository, root: &Path) {
        info!("Conforming Ansible role project.");

        // render standard vagrant stuff
        self.render_vagrantfile(root);
        self.render_playbook(root);
        self.render_ansible_cfg(root);
        self.render_galaxy_reqs(root);

        // docker compose for vms
        self.render_docker_compose(root);

        // git ignore
        self.render_gitignore(root);

        // python requirements (basically include the ansible egg)
        self.render_python_requirements(root);

        // travis
        self.render_travis_cfg(root);

        // makefile
        self.render_makefile(root);

        // alright time for tests/
        if !root.join("tests").is_dir() {
            info!("Creating tests/ directory.");
            fs::create_dir(root.join("tests")).map_err(|e| {
                error!("Unable to create tests/ directory: {}", e);
                exit(1)
            }).unwrap();
        }

        self.render_test_ansible_cfg(root);
        self.render_test_inventory(root);
        self.render_test_makefile(root);
        self.render_test_galaxy_reqs(root);
    }

    fn write_or_die<T>(&self, root: &Path, dest: &Path, template: T) where T: WritableTemplate {
        let base = dest.strip_prefix(root).unwrap();

        info!("Rendering {}", base.display());

        template.write(dest).map_err(|e| {
            error!("Unable to write {}: {}", dest.strip_prefix(root).unwrap().display(), e);
            exit(1)
        }).unwrap();
    }

    fn render_vagrantfile(&self, root: &Path) {
        self.write_or_die(root, &root.join("Vagrantfile"), templates::vagrant::Vagrantfile::new());
    }

    fn render_playbook(&self, root: &Path) {
        self.write_or_die(root, &root.join("vagrant.yml"), templates::vagrant::VagrantPlaybook::new()
            .docker()
            .python(self.python_version.as_str())
        );
    }

    fn render_ansible_cfg(&self, root: &Path) {
        self.write_or_die(root, &root.join("ansible.cfg"), templates::ansible::AnsibleConfig::new()
            .role_path(".ansible/galaxy-roles".to_string()));
    }

    fn render_galaxy_reqs(&self, root: &Path) {
        self.write_or_die(root, &root.join("requirements.yml"), templates::ansible::AnsibleRequirements::new()
            .role(GalaxyRole::new("naftulikay.vagrant-docker", Some("vagrant-docker"), None))
            .role(GalaxyRole::new("naftulikay.vagrant-python-dev", Some("vagrant-python-dev"), None))
        );
    }

    fn render_docker_compose(&self, root: &Path) {
        let mut cfg = templates::ansible::DockerComposeConfig::new();

        if self.enable_dind {
            cfg.with_dind();
        }

        if !self.disable_bionic {
            cfg.machines.push(DockerComposeMachine::new("bionic".into(), "naftulikay/bionic-vm:latest".into()).privileged());
        }

        if !self.disable_centos7 {
            cfg.machines.push(DockerComposeMachine::new("centos7".into(), "naftulikay/centos7-vm:latest".into()).privileged());
        }

        if !self.disable_juno {
            cfg.machines.push(DockerComposeMachine::new("juno".into(), "naftulikay/juno-vm:latest".into()).privileged());
        }

        if !self.disable_loki {
            cfg.machines.push(DockerComposeMachine::new("loki".into(), "naftulikay/loki-vm:latest".into()).privileged());
        }

        if !self.disable_trusty {
            cfg.machines.push(DockerComposeMachine::new("trusty".into(), "naftulikay/trusty-vm:latest".into()).privileged());
        }

        if !self.disable_xenial {
            cfg.machines.push(DockerComposeMachine::new("xenial".into(), "naftulikay/xenial-vm:latest".into()).privileged());
        }

        if cfg.machines.len() == 0 {
            error!("No machines found to add to docker-compose.yml.");
            exit(1)
        }

        cfg.write(&root.join("docker-compose.yml")).map_err(|e| {
            error!("Unable to render docker-compose.yml: {}", e);
            exit(1)
        }).unwrap();
    }

    fn render_gitignore(&self, root: &Path) {
        self.write_or_die(root, &root.join(".gitignore"), templates::git::GitIgnore::new().native().python());
    }

    fn render_python_requirements(&self, root: &Path) {
        let dest = root.join("requirements.txt");

        self.write_or_die(root, &root.join("requirements.txt"), templates::python::PythonRequirements::load(&dest)
            .unwrap_or(templates::python::PythonRequirements::new())
            .requirement("ansible".to_string())
        );
    }

    fn render_travis_cfg(&self, root: &Path) {
        self.write_or_die(root, &root.join(".travis.yml"), templates::ansible::TravisConfig::new());
    }

    fn render_makefile(&self, root: &Path) {
        self.write_or_die(root, &root.join("Makefile"), templates::ansible::Makefile::new());
    }

    fn render_test_makefile(&self, root: &Path) {
        self.write_or_die(root, &root.join("tests").join("Makefile"), templates::ansible::TestsMakefile::new());
    }

    fn render_test_ansible_cfg(&self, root: &Path) {
        self.write_or_die(root, &root.join("tests").join("ansible.cfg"), templates::ansible::AnsibleConfig::new()
            .role_path(".ansible/roles".to_string())
            .role_path(".ansible/galaxy-roles".to_string()));
    }

    fn render_test_inventory(&self, root: &Path) {
        let dir = &root.join("tests").join("inventory");

        if !dir.is_dir() {
            fs::create_dir(dir).map_err(|e| {
                error!("Unable to create {}: {}", dir.display(), e);
                exit(1)
            }).unwrap();
        }

        let mut template = templates::ansible::InventoryYaml::new();

        if !self.disable_bionic {
            template.add_host("bionic".to_string());
        }

        if !self.disable_centos7 {
            template.add_host("centos7".to_string());
        }

        if !self.disable_juno {
            template.add_host("juno".to_string());
        }

        if !self.disable_loki {
            template.add_host("loki".to_string());
        }

        if !self.disable_trusty {
            template.add_host("trusty".to_string());
        }

        if !self.disable_xenial {
            template.add_host("xenial".to_string());
        }

        if template.hosts.len() == 0 {
            error!("No hosts found to render into tests/inventory/hosts.yml.");
            exit(1)
        }

        self.write_or_die(root, &dir.join("hosts.yml"), template);
    }

    fn load_role_reqs(&self, root: &Path) -> Vec<String> {
        let role_path = &root.join("meta").join("main.yml");

        if role_path.exists() {
            if let Ok(file) = fs::File::open(role_path) {
                debug!("Successfully opened {} for Galaxy role dependencies.", role_path.strip_prefix(root).unwrap().display());

                match parse::ansible::RoleMetadata::load(io::BufReader::new(file)) {
                    Ok(meta) => meta.deps(),
                    Err(e) => {
                        warn!("Unable to parse role metadata from {}: {}", role_path.strip_prefix(root).unwrap().display(), e);
                        Vec::new()
                    }
                }
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }

    fn render_test_galaxy_reqs(&self, root: &Path) {
        let mut roles = self.load_role_reqs(root);
        roles.push("naftulikay.degoss".to_string());

        self.write_or_die(root, &root.join("tests").join("requirements.yml"),
            templates::ansible::AnsibleRequirements::new().with_roles(roles.iter().map(|s| {
                templates::ansible::GalaxyRole::from_src(s.clone())
            }).collect())
        );
    }
}
