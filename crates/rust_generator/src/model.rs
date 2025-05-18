use crate::utils::{access_root, escape_rust_name, replace_multiple_dashes};
use anyhow::Result;
use convert_case::Case;
use convert_case::Case::UpperCamel;
use convert_case::Casing;
use pulumi_gestalt_schema::model::*;

pub(crate) trait TypeExt {
    fn get_rust_type(&self, depth: usize) -> String;
    fn get_consts(&self) -> Vec<String>;
}

impl TypeExt for Type {
    fn get_rust_type(&self, depth: usize) -> String {
        match self {
            Type::Boolean => "bool".into(),
            Type::Integer => "i32".into(),
            Type::Number => "f64".into(),
            Type::String => "String".into(),
            Type::Array(type_) => {
                format!("Vec<{}>", type_.get_rust_type(depth))
            }
            Type::Object(type_) => {
                format!(
                    "std::collections::HashMap<String, {}>",
                    type_.get_rust_type(depth)
                )
            }
            Type::Ref(r) => match r {
                Ref::Type(tpe) => {
                    format!(
                        "{}types::{}",
                        access_root(depth),
                        tpe.get_rust_absolute_name()
                    )
                }
                Ref::Archive => "String".to_string(), //FIXME
                Ref::Asset => "String".to_string(),   //FIXME
                Ref::Any => "String".to_string(),     //FIXME
            },
            Type::Option(type_) => format!("Option<{}>", type_.get_rust_type(depth)),
            Type::DiscriminatedUnion(refs) => format!(
                "pulumi_gestalt_rust::OneOf{}<{}>",
                refs.len(),
                refs.iter()
                    .map(|r| r.get_rust_type(depth))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Type::ConstString(s) => {
                let prefix = if depth > 0 {
                    "super::".repeat(depth)
                } else {
                    "self::".to_string()
                };
                format!("{}constants::ConstString{}", prefix, s.to_case(UpperCamel)).to_string()
            }
        }
    }

    fn get_consts(&self) -> Vec<String> {
        match self {
            Type::Boolean => vec![],
            Type::Integer => vec![],
            Type::Number => vec![],
            Type::String => vec![],
            Type::ConstString(s) => vec![s.clone()],
            Type::Ref(_) => vec![],
            Type::Array(t) => t.get_consts(),
            Type::Object(o) => o.get_consts(),
            Type::Option(o) => o.get_consts(),
            Type::DiscriminatedUnion(_) => vec![],
        }
    }
}

pub(crate) trait InputPropertyExt {
    fn get_rust_argument_name(&self) -> String;
}

impl InputPropertyExt for InputProperty {
    fn get_rust_argument_name(&self) -> String {
        escape_rust_name(ElementId::create_valid_id(self.name.as_str()).as_str()).into()
    }
}

pub(crate) trait OutputPropertyExt {
    fn get_rust_argument_name(&self) -> String;
}

impl OutputPropertyExt for OutputProperty {
    fn get_rust_argument_name(&self) -> String {
        escape_rust_name(ElementId::create_valid_id(self.name.as_str()).as_str()).into()
    }
}

pub(crate) trait GlobalTypePropertyExt {
    fn get_field_name(&self) -> String;
}

impl GlobalTypePropertyExt for GlobalTypeProperty {
    fn get_field_name(&self) -> String {
        escape_rust_name(
            &self
                .name
                .clone()
                .from_case(Case::Camel)
                .to_case(Case::Snake),
        )
        .to_string()
    }
}

pub(crate) trait ElementIdExt {
    fn get_rust_struct_name(&self) -> String;
    fn get_rust_absolute_name(&self) -> String;
    fn get_rust_function_name(&self) -> String;
    fn get_rust_package_name(&self) -> String;
    fn get_rust_namespace_name(&self) -> String;
    fn create_valid_id(s: &str) -> String;
    fn new(raw: &str) -> Result<Self>
    where
        Self: Sized;
}

impl ElementIdExt for ElementId {
    fn get_rust_struct_name(&self) -> String {
        self.name.clone().to_case(Case::Pascal)
    }

    fn get_rust_absolute_name(&self) -> String {
        let mut parts = self.namespace.clone();
        parts.push(self.name.clone().to_case(Case::Pascal));
        parts.join("::")
    }

    fn get_rust_function_name(&self) -> String {
        self.name.clone().from_case(UpperCamel).to_case(Case::Snake)
    }

    fn get_rust_package_name(&self) -> String {
        escape_rust_name(&self.name.clone().to_case(Case::Snake)).to_string()
    }

    fn get_rust_namespace_name(&self) -> String {
        let mut vec = self.namespace.clone();
        vec.push(self.name.clone());
        Self::create_valid_id(&vec.join("-"))
    }

    fn create_valid_id(s: &str) -> String {
        let result: String = s
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    format!("-{}", c.to_lowercase())
                } else if !c.is_alphanumeric() {
                    "-".to_string()
                } else {
                    c.to_string()
                }
            })
            .collect();

        let result = replace_multiple_dashes(&result);
        let result = result.trim_matches('-').to_string();
        
        result.replace("-", "_")
    }

    fn new(raw: &str) -> Result<Self> {
        let raw = raw.replace("%2F", "/");
        let parts: Vec<&str> = raw.split(':').collect();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("Cannot generate element id from [{raw}]"));
        }
        let name = parts[2].to_string();
        let namespace = parts[1].split('/').collect::<Vec<_>>();

        let mut final_namespaces = Vec::new();

        for (i, s) in namespace.iter().enumerate() {
            if i == 0 && s == &"index" {
                continue;
            }
            if i == namespace.len() - 1 && s.to_lowercase() == name.to_lowercase() {
                continue;
            }
            if s.is_empty() {
                continue;
            }

            final_namespaces.push(escape_rust_name(&s.replace("-", "_")).to_string());
        }

        Ok(ElementId {
            namespace: final_namespaces,
            name,
            raw: raw.to_string(),
        })
    }
}
