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
