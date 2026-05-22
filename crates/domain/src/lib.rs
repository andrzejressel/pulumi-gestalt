pub use pulumi_gestalt_model::{PulumiValue, PulumiValueContent};
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

pub struct ResourceFields {
    pub object: HashMap<FieldName, PulumiValue>,
    pub is_in_preview: bool,
}

impl ResourceFields {
    pub fn get_field_value(&self, field_name: &FieldName) -> PulumiValue {
        match (self.object.get(field_name), self.is_in_preview) {
            (Some(existing_value), _) => existing_value.clone(),
            (None, true) => PulumiValue::nothing(),
            (None, false) => PulumiValue::none(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FieldName, PulumiValue, PulumiValueContent, ResourceFields};
    use std::collections::HashMap;

    #[test]
    fn should_get_existing_field_value() {
        let mut object = HashMap::new();
        object.insert(
            FieldName::from("existing_field"),
            PulumiValue {
                content: PulumiValueContent::String("existing_value".to_string()),
                secret: false,
                dependencies: Default::default(),
            },
        );

        let resource_fields = ResourceFields {
            object,
            is_in_preview: false,
        };

        let field_name = FieldName::from("existing_field");
        let result = resource_fields.get_field_value(&field_name);

        assert_eq!(
            result,
            PulumiValue {
                content: PulumiValueContent::String("existing_value".to_string()),
                secret: false,
                dependencies: Default::default(),
            }
        );
    }

    #[test]
    fn if_is_in_preview_return_nothing() {
        let resource_fields = ResourceFields {
            object: HashMap::new(),
            is_in_preview: true,
        };

        let field_name = FieldName::from("non_existing_field");
        let result = resource_fields.get_field_value(&field_name);

        assert!(matches!(result.content, PulumiValueContent::Nothing));
    }

    #[test]
    fn if_not_in_preview_return_null() {
        let resource_fields = ResourceFields {
            object: HashMap::new(),
            is_in_preview: false,
        };

        let field_name = FieldName::from("non_existing_field");
        let result = resource_fields.get_field_value(&field_name);

        assert!(matches!(result.content, PulumiValueContent::None));
    }
}
