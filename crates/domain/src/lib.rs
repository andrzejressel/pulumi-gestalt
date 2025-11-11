use serde_json::Value;
use std::collections::HashMap;

pub mod connector;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldName(String);

impl FieldName {
    pub fn as_string(&self) -> &String {
        &self.0
    }

    pub fn get_inner(self) -> String {
        self.0
    }
}

impl From<String> for FieldName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for FieldName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<&String> for FieldName {
    fn from(value: &String) -> Self {
        Self(value.to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExistingNodeValue {
    pub value: Value,
    pub secret: bool,
}

impl ExistingNodeValue {
    pub fn new<T: Into<Value>>(value: T, secret: bool) -> Self {
        Self {
            value: value.into(),
            secret,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum NodeValue {
    Nothing, // preview
    Exists(ExistingNodeValue),
}

impl NodeValue {
    pub fn exists<T: Into<Value>>(value: T, secret: bool) -> Self {
        Self::Exists(ExistingNodeValue::new(value, secret))
    }
}

pub struct ResourceFields {
    pub object: HashMap<FieldName, ExistingNodeValue>,
    pub is_in_preview: bool,
}

impl ResourceFields {
    pub fn get_field_value(&self, field_name: &FieldName) -> NodeValue {
        match (self.object.get(field_name), self.is_in_preview) {
            (Some(existing_value), _) => {
                NodeValue::exists(existing_value.value.clone(), existing_value.secret)
            }
            (None, true) => NodeValue::Nothing,
            (None, false) => NodeValue::exists(Value::Null, false),
        }
    }
}

mod tests {
    use super::{ExistingNodeValue, FieldName, NodeValue, ResourceFields};
    use NodeValue::Nothing;
    use serde_json::Value::Null;
    use std::collections::HashMap;

    #[test]
    fn should_get_existing_field_value() {
        let mut object = HashMap::new();
        object.insert(
            FieldName::from("existing_field"),
            ExistingNodeValue::new("existing_value", false),
        );

        let resource_fields = ResourceFields {
            object,
            is_in_preview: false,
        };

        let field_name = FieldName::from("existing_field");
        let result = resource_fields.get_field_value(&field_name);

        assert_eq!(result, NodeValue::exists("existing_value", false));
    }

    #[test]
    fn if_is_in_preview_return_nothing() {
        let resource_fields = ResourceFields {
            object: HashMap::new(),
            is_in_preview: true,
        };

        let field_name = FieldName::from("non_existing_field");
        let result = resource_fields.get_field_value(&field_name);

        assert_eq!(result, Nothing);
    }

    #[test]
    fn if_not_in_preview_return_null() {
        let resource_fields = ResourceFields {
            object: HashMap::new(),
            is_in_preview: false,
        };

        let field_name = FieldName::from("non_existing_field");
        let result = resource_fields.get_field_value(&field_name);

        assert_eq!(result, NodeValue::exists(Null, false));
    }
}
