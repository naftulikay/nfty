use super::*;

use serde_yaml;

static GALAXY_MANIFEST: &'static str = r###"
---
galaxy_info:
  name: role_name
  author: naftulikay
  src: naftulikay.role_name
dependencies:
  - simple
  - role: complex
"###;

#[test]
fn test_galaxy_manifest_parsing() {
    let parsed: RoleMetadata = serde_yaml::from_str(GALAXY_MANIFEST).unwrap();

    assert_eq!("role_name", parsed.galaxy_info.name);
    assert_eq!("naftulikay", parsed.galaxy_info.author);
    assert_eq!("naftulikay.role_name", parsed.galaxy_info.src);

    assert_eq!(2, parsed.dependencies.len());

    let deps = parsed.deps();

    assert!(deps.contains(&"simple".to_string()));
    assert!(deps.contains(&"complex".to_string()));
}
