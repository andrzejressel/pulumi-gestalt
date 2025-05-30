use crate::code_generation::YamlFile;
use crate::code_generation::yaml::model::Expression;
use crate::code_generation::yaml::model::{Example, Resource};
use crate::code_generation::yaml::yaml_model::YamlExpression;
use crate::utils::reformat_code;
use pulumi_gestalt_schema::model::ElementId;
use std::collections::BTreeMap;

//language=YAML
pub const YAML: &str = r#"
    resources:
      example:
        type: yamltests:KeylessCertificate
        properties:
          port: 24008
"#;

pub fn get_yaml_file() -> YamlFile {
    use crate::code_generation::yaml::yaml_model::{YamlFile, YamlResource};

    YamlFile {
        resources: {
            let mut resources = BTreeMap::new();
            resources.insert(
                "example".to_string(),
                YamlResource {
                    type_: "yamltests:KeylessCertificate".to_string(),
                    name: None,
                    properties: {
                        let mut properties = BTreeMap::new();
                        properties.insert("port".to_string(), YamlExpression::Number(24008f64));
                        properties
                    },
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
                "example".to_string(),
                Resource {
                    type_: ElementId::new("yamltests:index/keylessCertificate:KeylessCertificate")
                        .unwrap(),
                    // type_: "yamltests:AccessMutualTlsCertificate".to_string(),
                    name: None,
                    properties: {
                        let mut props = BTreeMap::new();
                        props.insert("port".to_string(), Expression::Integer(24008));
                        props
                    },
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
            let example = keyless_certificate::create(
                "example",
                KeylessCertificateArgs::builder()
                    .port(24008)
                    .build_struct(),
            );
        }
    "#,
    )
    .unwrap()
}
