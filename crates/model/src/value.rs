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
    fn plain(content: PulumiValueContent) -> Self {
        Self {
            content,
            secret: false,
            dependencies: HashSet::new(),
        }
    }
}

impl From<String> for PulumiValue {
    fn from(value: String) -> Self {
        Self::plain(PulumiValueContent::String(value))
    }
}

impl From<&str> for PulumiValue {
    fn from(value: &str) -> Self {
        Self::plain(PulumiValueContent::String(value.to_string()))
    }
}

impl From<i32> for PulumiValue {
    fn from(value: i32) -> Self {
        Self::plain(PulumiValueContent::Integer(value))
    }
}

impl From<f64> for PulumiValue {
    fn from(value: f64) -> Self {
        Self::plain(PulumiValueContent::Number(value))
    }
}

impl From<bool> for PulumiValue {
    fn from(value: bool) -> Self {
        Self::plain(PulumiValueContent::Boolean(value))
    }
}
