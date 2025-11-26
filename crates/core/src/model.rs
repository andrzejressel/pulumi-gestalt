
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
