use super::*;

#[test]
fn test_docker_playbook() {
    let rendered = VagrantPlaybook::new().docker().render().unwrap();

    assert_eq!(
        include_str!("fixtures/playbooks/docker.yml").trim(),
        rendered.trim()
    );
}

#[test]
fn test_go_playbook() {
    let rendered = VagrantPlaybook::new()
        .go("GO_PACKAGE", "GO_VERSION")
        .render()
        .unwrap();

    assert_eq!(
        include_str!("fixtures/playbooks/go.yml").trim(),
        rendered.trim()
    );
}

#[test]
fn test_hybrid_playbook() {
    let rendered = VagrantPlaybook::new()
        .docker()
        .python("PYTHON_VERSION")
        .render()
        .unwrap();

    assert_eq!(
        include_str!("fixtures/playbooks/hybrid.yml").trim(),
        rendered.trim()
    );
}

#[test]
fn test_java_playbook() {
    let rendered = VagrantPlaybook::new().java().render().unwrap();

    assert_eq!(
        include_str!("fixtures/playbooks/java.yml").trim(),
        rendered.trim()
    );
}

#[test]
fn test_node_playbook() {
    let rendered = VagrantPlaybook::new()
        .node("NODE_VERSION")
        .render()
        .unwrap();

    assert_eq!(
        include_str!("fixtures/playbooks/node.yml").trim(),
        rendered.trim()
    );
}

#[test]
fn test_python_playbook() {
    let rendered = VagrantPlaybook::new()
        .python("PYTHON_VERSION")
        .render()
        .unwrap();

    assert_eq!(
        include_str!("fixtures/playbooks/python.yml").trim(),
        rendered.trim()
    );
}

#[test]
fn test_ruby_playbook() {
    let rendered = VagrantPlaybook::new()
        .ruby("RUBY_VERSION")
        .render()
        .unwrap();

    assert_eq!(
        include_str!("fixtures/playbooks/ruby.yml").trim(),
        rendered.trim()
    );
}

#[test]
fn test_rust_playbook() {
    let rendered = VagrantPlaybook::new().rust().render().unwrap();

    assert_eq!(
        include_str!("fixtures/playbooks/rust.yml").trim(),
        rendered.trim()
    );
}
