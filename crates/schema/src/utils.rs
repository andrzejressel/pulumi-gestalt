use crate::model::ElementId;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::sync::LazyLock;

pub(crate) fn replace_multiple_dashes(s: &str) -> String {
    let re = Regex::new("-+").unwrap();
    let result = re.replace_all(s, "-");
    result.to_string()
}

pub(crate) fn sanitize_identifier(input: &str) -> String {
    // Filter characters that are valid for an identifier in Rust
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_') // Keep letters, digits, and underscores
        .collect()
}

pub(crate) fn fix_description(s: Option<String>, element_id: Option<ElementId>) -> Option<String> {
    s.map(|s| fix_pulumi_docker_docs(s, element_id))
}

static DOCKER_SERVICE_REPLACEMENTS: LazyLock<HashMap<ElementId, Vec<(&str, &str)>>> =
    LazyLock::new(|| {
        HashMap::from([
            (
                ElementId::new("docker:index/service:Service").unwrap(),
                vec![(
                    include_str!("dockerfixes/service/1_original.md"),
                    include_str!("dockerfixes/service/1_fixed.md"),
                )],
            ),
            (
                ElementId::new("docker:index/getPlugin:getPlugin").unwrap(),
                vec![(
                    include_str!("dockerfixes/getPlugin/1_original.md"),
                    include_str!("dockerfixes/getPlugin/1_fixed.md"),
                )],
            ),
            (
                ElementId::new("docker:index/network:Network").unwrap(),
                vec![(
                    include_str!("dockerfixes/network/1_original.md"),
                    include_str!("dockerfixes/network/1_fixed.md"),
                )],
            ),
            (
                ElementId::new("docker:index/secret:Secret").unwrap(),
                vec![(
                    include_str!("dockerfixes/secret/1_original.md"),
                    include_str!("dockerfixes/secret/1_fixed.md"),
                )],
            ),
            (
                ElementId::new("docker:index/serviceConfig:ServiceConfig").unwrap(),
                vec![(
                    include_str!("dockerfixes/serviceConfig/1_original.md"),
                    include_str!("dockerfixes/serviceConfig/1_fixed.md"),
                )],
            ),
            (
                ElementId::new("docker:index/container:Container").unwrap(),
                vec![(
                    include_str!("dockerfixes/container/1_original.md"),
                    include_str!("dockerfixes/container/1_fixed.md"),
                )],
            ),
        ])
    });

fn fix_pulumi_docker_docs(s: String, element_id: Option<ElementId>) -> String {
    if let Some(id) = element_id {
        let replacement = &DOCKER_SERVICE_REPLACEMENTS;
        if let Some(replacements) = replacement.get(&id) {
            for (origin, fixed) in replacements {
                if s.contains(origin) {
                    return fixed.to_string();
                }
            }
            fs::write("error.md", s).unwrap();
            panic!(
                "ElementId {:?} does not have valid replacement. Original markdown was saved to error.md",
                id
            );
        }
    }

    s
}
