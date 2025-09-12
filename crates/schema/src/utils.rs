use crate::model::ElementId;
use std::collections::HashMap;
use std::sync::LazyLock;

pub(crate) fn fix_description(s: Option<String>, element_id: &ElementId) -> Option<String> {
    s.map(|s| fix_pulumi_docker_docs(s, element_id))
}

static DOCKER_SERVICE_REPLACEMENTS: LazyLock<HashMap<ElementId, Vec<(&str, &str)>>> =
    LazyLock::new(|| {
        HashMap::from([
            (
                ElementId::new("docker:index/service:Service").expect("Invalid element ID"),
                vec![(
                    include_str!("dockerfixes/service/1_original.md"),
                    include_str!("dockerfixes/service/1_fixed.md"),
                )],
            ),
            (
                ElementId::new("docker:index/getPlugin:getPlugin").expect("Invalid element ID"),
                vec![(
                    include_str!("dockerfixes/getPlugin/1_original.md"),
                    include_str!("dockerfixes/getPlugin/1_fixed.md"),
                )],
            ),
            (
                ElementId::new("docker:index/network:Network").expect("Invalid element ID"),
                vec![(
                    include_str!("dockerfixes/network/1_original.md"),
                    include_str!("dockerfixes/network/1_fixed.md"),
                )],
            ),
            (
                ElementId::new("docker:index/secret:Secret").expect("Invalid element ID"),
                vec![(
                    include_str!("dockerfixes/secret/1_original.md"),
                    include_str!("dockerfixes/secret/1_fixed.md"),
                )],
            ),
            (
                ElementId::new("docker:index/serviceConfig:ServiceConfig").expect("Invalid element ID"),
                vec![(
                    include_str!("dockerfixes/serviceConfig/1_original.md"),
                    include_str!("dockerfixes/serviceConfig/1_fixed.md"),
                )],
            ),
            (
                ElementId::new("docker:index/container:Container").expect("Invalid element ID"),
                vec![(
                    include_str!("dockerfixes/container/1_original.md"),
                    include_str!("dockerfixes/container/1_fixed.md"),
                )],
            ),
        ])
    });

fn fix_pulumi_docker_docs(s: String, element_id: &ElementId) -> String {
    let replacement = &DOCKER_SERVICE_REPLACEMENTS;
    if let Some(replacements) = replacement.get(element_id) {
        for (origin, fixed) in replacements {
            if s.contains(origin) {
                return fixed.to_string();
            }
        }
    }

    s
}
