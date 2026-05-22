use serde_json::Value as JsonValue;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub enum PulumiValueContent {
    String(String),
    Integer(i32),
    Number(f64),
    Boolean(bool),
    Array(Vec<PulumiValue>),
    Object(Vec<(String, PulumiValue)>),
    None,
    Nothing,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PulumiValue {
    pub content: PulumiValueContent,
    pub secret: bool,
    pub dependencies: HashSet<String>,
}

impl PulumiValue {
    pub fn nothing() -> Self {
        Self {
            content: PulumiValueContent::Nothing,
            secret: false,
            dependencies: HashSet::new(),
        }
    }

    pub fn none() -> Self {
        Self {
            content: PulumiValueContent::None,
            secret: false,
            dependencies: HashSet::new(),
        }
    }

    pub fn from_json(value: JsonValue, secret: bool) -> Self {
        let content = match value {
            JsonValue::Null => PulumiValueContent::None,
            JsonValue::Bool(boolean) => PulumiValueContent::Boolean(boolean),
            JsonValue::Number(number) => {
                if let Some(integer) = number.as_i64() {
                    PulumiValueContent::Integer(
                        i32::try_from(integer)
                            .expect("i64 value is outside supported i32 range for Pulumi integers"),
                    )
                } else {
                    PulumiValueContent::Number(
                        number
                            .as_f64()
                            .expect("serde_json::Number must be convertible to f64"),
                    )
                }
            }
            JsonValue::String(string) => PulumiValueContent::String(string),
            JsonValue::Array(values) => PulumiValueContent::Array(
                values
                    .into_iter()
                    .map(|v| PulumiValue::from_json(v, false))
                    .collect(),
            ),
            JsonValue::Object(values) => PulumiValueContent::Object(
                values
                    .into_iter()
                    .map(|(key, value)| (key, PulumiValue::from_json(value, false)))
                    .collect(),
            ),
        };

        Self {
            content,
            secret,
            dependencies: HashSet::new(),
        }
    }

    pub fn to_json(&self) -> JsonValue {
        match &self.content {
            PulumiValueContent::String(value) => JsonValue::String(value.clone()),
            PulumiValueContent::Integer(value) => JsonValue::from(*value),
            PulumiValueContent::Number(value) => JsonValue::from(*value),
            PulumiValueContent::Boolean(value) => JsonValue::from(*value),
            PulumiValueContent::Array(values) => {
                JsonValue::Array(values.iter().map(PulumiValue::to_json).collect())
            }
            PulumiValueContent::Object(values) => JsonValue::Object(
                values
                    .iter()
                    .map(|(key, value)| (key.clone(), value.to_json()))
                    .collect(),
            ),
            PulumiValueContent::None | PulumiValueContent::Nothing => JsonValue::Null,
        }
    }
}
