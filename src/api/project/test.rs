use super::*;

#[test]
fn test_ssh_project_regex() {
    assert!(SSH_PROVIDER.is_match(&"naftulikay/titan"));
    assert!(SSH_PROVIDER.is_match(&"naftulikay/titan.git"));
    assert!(SSH_PROVIDER.is_match(&"github.com:naftulikay/titan"));
    assert!(SSH_PROVIDER.is_match(&"github.com:naftulikay/titan.git"));
    assert!(SSH_PROVIDER.is_match(&"git@github.com:naftulikay/titan"));
    assert!(SSH_PROVIDER.is_match(&"git@github.com:naftulikay/titan.git"));

    // with a dash please
    assert!(SSH_PROVIDER.is_match(&"naftulikay/ansible-role-rust-dev"));

    let captures = SSH_PROVIDER.captures(&"naftulikay/ansible-role-rust-dev").unwrap();
    assert_eq!("naftulikay", captures.name("owner").unwrap().as_str());
    assert_eq!("ansible-role-rust-dev", captures.name("repository").unwrap().as_str());
}

#[test]
fn test_project_from_https() {
    let repositories = ["nfty", "ansible-role-vim-personal", "phatnoise.rs"];

    for repository in repositories.iter() {
        let project = Project::from(&format!("https://github.com/naftulikay/{}", &repository)).unwrap();
        assert_eq!(project.protocol(), &Protocol::Https);
        assert_eq!(project.host(), "github.com");
        assert_eq!(project.owner(), "naftulikay");
        assert_eq!(&project.repository(), repository);
    }
}

#[test]
fn test_project_from_ssh() {
    let repositories = ["nfty", "ansible-role-vim-personal", "phatnoise.rs"];

    for repository in repositories.iter() {
        // user and repository only
        let project = Project::from(&format!("naftulikay/{}", &repository)).unwrap();
        assert_eq!(project.protocol(), &Protocol::Ssh);
        assert_eq!(project.user(), "git");
        assert_eq!(project.host(), "github.com");
        assert_eq!(project.owner(), "naftulikay");
        assert_eq!(&project.repository(), repository);

        // hostname, owner, repository
        let project = Project::from(&format!("github.com:naftulikay/{}", &repository)).unwrap();
        assert_eq!(project.protocol(), &Protocol::Ssh);
        assert_eq!(project.user(), "git");
        assert_eq!(project.host(), "github.com");
        assert_eq!(project.owner(), "naftulikay");
        assert_eq!(&project.repository(), repository);

        // user, hostname, owner, repository
        let project = Project::from(&format!("git@github.com:naftulikay/{}", &repository)).unwrap();
        assert_eq!(project.protocol(), &Protocol::Ssh);
        assert_eq!(project.user(), "git");
        assert_eq!(project.host(), "github.com");
        assert_eq!(project.owner(), "naftulikay");
        assert_eq!(&project.repository(), repository);

        // user, hostname, owner, repository, postfix
        let project = Project::from(&format!("git@github.com:naftulikay/{}.git", &repository)).unwrap();
        assert_eq!(project.protocol(), &Protocol::Ssh);
        assert_eq!(project.user(), "git");
        assert_eq!(project.host(), "github.com");
        assert_eq!(project.owner(), "naftulikay");
        assert_eq!(&project.repository(), repository);
    }
}
