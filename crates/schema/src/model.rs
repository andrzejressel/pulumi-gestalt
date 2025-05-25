use anyhow::{Context, Result};
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub enum Type {
    Boolean,
    Integer,
    Number,
    String,
    Array(Box<Type>),
    Object(Box<Type>),
    Ref(Ref),
    Option(Box<Type>),
    DiscriminatedUnion(Vec<Type>),
    ConstString(String),
}

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub struct InputProperty {
    pub name: String,
    pub r#type: Type,
    pub description: Option<String>,
}

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub struct OutputProperty {
    pub name: String,
    pub r#type: Type,
    pub description: Option<String>,
}

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub struct GlobalTypeProperty {
    pub name: String,
    pub r#type: Type,
    pub description: Option<String>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct GlobalType {
    pub element_id: ElementId,
    pub value: GlobalTypeValue,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum GlobalTypeValue {
    Object(Option<String>, Vec<GlobalTypeProperty>),
    StringEnum(Option<String>, Vec<StringEnumElement>),
    NumberEnum(Option<String>, Vec<NumberEnumElement>),
    IntegerEnum(Option<String>, Vec<IntegerEnumElement>),
}

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub struct StringEnumElement {
    pub name: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct NumberEnumElement {
    pub name: String,
    pub value: f64,
    pub description: Option<String>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct IntegerEnumElement {
    pub name: String,
    pub value: i64,
    pub description: Option<String>,
}

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub struct Resource {
    pub element_id: ElementId,
    // pub name: String,
    pub description: Option<String>,
    pub input_properties: Vec<InputProperty>,
    pub output_properties: Vec<OutputProperty>,
}

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub struct Function {
    pub element_id: ElementId,
    // pub name: String,
    pub description: Option<String>,
    pub input_properties: Vec<InputProperty>,
    pub output_properties: Vec<OutputProperty>,
}

#[derive(Debug, PartialEq)]
pub struct Package {
    pub name: String,
    pub display_name: Option<String>,
    pub plugin_download_url: Option<String>,
    pub version: String,
    pub resources: BTreeMap<ElementId, Rc<Resource>>,
    pub functions: BTreeMap<ElementId, Rc<Function>>,
    pub types: BTreeMap<ElementId, Rc<GlobalType>>,

    pub resource_name_map: HashMap<String, Rc<Resource>>,
    pub function_name_map: HashMap<String, Rc<Function>>,
    pub all_types: BTreeMap<ElementId, Rc<GlobalType>>,
}

impl Package {
    pub fn new(
        name: String,
        display_name: Option<String>,
        plugin_download_url: Option<String>,
        version: String,
        resources: BTreeMap<ElementId, Resource>,
        functions: BTreeMap<ElementId, Function>,
        types: BTreeMap<ElementId, GlobalType>,
    ) -> Self {
        let mut resource_name_map = HashMap::new();
        let mut new_resources = BTreeMap::new();
        for (element_id, resource) in resources {
            let mut chunks = Vec::new();
            chunks.push(name.clone());
            chunks.extend(element_id.namespace.clone());
            chunks.push(element_id.name.clone());
            let rc = Rc::new(resource);
            new_resources.insert(element_id, rc.clone());
            let name = chunks.join(":");

            resource_name_map.insert(name.clone(), rc.clone());
        }

        let mut new_function = BTreeMap::new();
        let mut function_name_map = HashMap::new();
        for (element_id, function) in functions {
            let mut chunks = Vec::new();
            chunks.push(name.clone());
            chunks.extend(element_id.namespace.clone());
            chunks.push(element_id.name.clone());

            let rc = Rc::new(function);
            new_function.insert(element_id, rc.clone());
            let name = chunks.join(":");
            function_name_map.insert(name.clone(), rc.clone());
        }

        let mut new_types = BTreeMap::new();
        for (element_id, t) in types {
            let rc = Rc::new(t);
            new_types.insert(element_id.clone(), rc.clone());
        }

        Self {
            name,
            display_name,
            version,
            plugin_download_url,
            resources: new_resources,
            functions: new_function,
            types: new_types.clone(),
            resource_name_map,
            function_name_map,
            all_types: new_types,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub enum Ref {
    Type(ElementId),
    Archive,
    Asset,
    Any,
}

#[derive(Clone, Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub struct ElementId {
    pub namespace: Vec<String>,
    pub name: String,
    pub raw: String,
}

impl Ref {
    pub fn new(raw: &str) -> Result<Self> {
        if raw == "pulumi.json#/Archive" {
            Ok(Ref::Archive)
        } else if raw == "pulumi.json#/Asset" {
            Ok(Ref::Asset)
        } else if raw == "pulumi.json#/Any" {
            Ok(Ref::Any)
        } else if raw.starts_with("#/types/") {
            Ok(Ref::Type(ElementId::new(
                raw.strip_prefix("#/types/")
                    .context(format!("Cannot strip types prefix from {raw}"))?,
            )?))
        } else {
            Err(anyhow::anyhow!("Cannot generate ref from [{raw}]."))
        }
    }
}

impl ElementId {
    pub fn new(raw: &str) -> Result<Self> {
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

            final_namespaces.push(s.to_string());
        }

        Ok(ElementId {
            namespace: final_namespaces,
            name,
            raw: raw.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::ElementId;

    #[test]
    fn extract_namespace_from_command() {
        let id = "command:remote:Connection";
        assert_eq!(
            ElementId::new(id).unwrap(),
            ElementId {
                namespace: vec!["remote".to_string()],
                name: "Connection".to_string(),
                raw: id.to_string(),
            }
        );
    }

    #[test]
    fn extract_namespace_from_random() {
        let id = "random:index/randomBytes:RandomBytes";
        assert_eq!(
            ElementId::new(id).unwrap(),
            ElementId {
                namespace: vec![],
                name: "RandomBytes".to_string(),
                raw: id.to_string(),
            }
        );
    }

    #[test]
    fn perform_escaping() {
        let id = "docker:index%2FContainerPort:ContainerPort";
        assert_eq!(
            ElementId::new(id).unwrap(),
            ElementId {
                namespace: vec![],
                name: "ContainerPort".to_string(),
                raw: "docker:index/ContainerPort:ContainerPort".to_string(),
            }
        );
    }

    #[test]
    fn should_handle_without_namespace() {
        let id = "mypkg::BastionShareableLink";
        assert_eq!(
            ElementId::new(id).unwrap(),
            ElementId {
                namespace: vec![],
                name: "BastionShareableLink".to_string(),
                raw: id.to_string(),
            }
        );
    }

    #[test]
    fn should_handle_deeply_nested() {
        let id = "foo-bar:deeply/nested/module:Resource";
        assert_eq!(
            ElementId::new(id).unwrap(),
            ElementId {
                namespace: vec![
                    "deeply".to_string(),
                    "nested".to_string(),
                    "module".to_string()
                ],
                name: "Resource".to_string(),
                raw: id.to_string(),
            }
        )
    }

    #[test]
    fn should_handle_nesting() {
        let id = "azure:frontdoor/frontdoor:Frontdoor";
        assert_eq!(
            ElementId::new(id).unwrap(),
            ElementId {
                namespace: vec!["frontdoor".to_string()],
                name: "Frontdoor".to_string(),
                raw: id.to_string(),
            }
        )
    }
}
