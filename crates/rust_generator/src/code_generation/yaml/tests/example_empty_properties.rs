use crate::code_generation::YamlFile;
use crate::code_generation::yaml::model::{Example, Resource};
use crate::utils::reformat_code;
use pulumi_gestalt_schema::model::ElementId;
use std::collections::BTreeMap;

//language=YAML
pub const YAML: &str = r#"
    resources:
        myCert:
            type: yamltests:AccessMutualTlsCertificate
            name: my_cert
"#;

pub fn get_yaml_file() -> YamlFile {
    use crate::code_generation::yaml::yaml_model::{YamlFile, YamlResource};

    YamlFile {
        resources: {
            let mut resources = BTreeMap::new();
            resources.insert(
                "myCert".to_string(),
                YamlResource {
                    type_: "yamltests:AccessMutualTlsCertificate".to_string(),
                    name: Some("my_cert".to_string()),
                    properties: BTreeMap::new(),
                },
            );
            resources
        },
        variables: BTreeMap::new(),
    }
}

pub fn get_model() -> Example {
    Example {
        resources: {
            let mut map = BTreeMap::new();
            map.insert(
                "myCert".to_string(),
                Resource {
                    type_: ElementId::new(
                        "yamltests:index/accessMutualTlsCertificate:AccessMutualTlsCertificate",
                    )
                    .unwrap(),
                    // type_: "yamltests:AccessMutualTlsCertificate".to_string(),
                    name: Some("my_cert".to_string()),
                    properties: BTreeMap::new(),
                },
            );
            map
        },
        variables: BTreeMap::new(),
    }
}

// language=Rust
pub fn get_rust_code() -> String {
    reformat_code(
        r#"
    use pulumi_gestalt_rust::Output;
    use pulumi_gestalt_rust::{add_export, pulumi_main};
    #[pulumi_main]
    fn test_main() -> Result<(), Error> {
        let myCert = access_mutual_tls_certificate::create(
            "myCert",
            AccessMutualTlsCertificateArgs::builder()
                .build_struct(),
        );
    }
    "#,
    )
    .unwrap()
}
