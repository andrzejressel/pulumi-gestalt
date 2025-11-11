use pulumi_gestalt_domain::NodeValue;
use serde_json::Value;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum MaybeNodeValue {
    NotYetCalculated,
    Set(NodeValue),
}

impl MaybeNodeValue {
    #[cfg(test)]
    pub(crate) fn set_value(value: Value, secret: bool) -> Self {
        Self::Set(NodeValue::exists(value, secret))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionName(String);

impl From<FunctionName> for String {
    fn from(val: FunctionName) -> Self {
        val.0
    }
}

impl From<String> for FunctionName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for FunctionName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
