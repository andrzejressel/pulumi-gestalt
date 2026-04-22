use crate::{Context, Output};
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Wrapper for either static value or [Output]
pub enum Input<T> {
    StaticValue(T),
    Output(Output<T>),
}

impl<T: Serialize> Input<T> {
    #[doc(hidden)]
    pub fn get_output(self, engine: &Context) -> Output<T> {
        match self {
            Input::StaticValue(value) => engine.new_output(&value),
            Input::Output(output) => output,
        }
    }
}

impl<T> From<Output<T>> for Input<T> {
    fn from(output: Output<T>) -> Self {
        Input::Output(output)
    }
}

impl<T: Default + Serialize> Default for Input<T> {
    fn default() -> Self {
        Input::StaticValue(Default::default())
    }
}

impl<T: Serialize> From<T> for Input<T> {
    fn from(value: T) -> Input<T> {
        Input::StaticValue(value)
    }
}

impl<T: Serialize> From<T> for Input<Option<T>> {
    fn from(value: T) -> Self {
        Input::StaticValue(Some(value))
    }
}

impl<T: Serialize + DeserializeOwned> From<Output<T>> for Input<Option<T>> {
    fn from(output: Output<T>) -> Self {
        Input::Output(output.map(|v| Some(v)))
    }
}

impl From<&str> for Input<String> {
    fn from(value: &str) -> Self {
        Input::StaticValue(value.to_string())
    }
}

impl From<&str> for Input<Option<String>> {
    fn from(value: &str) -> Self {
        Input::StaticValue(Some(value.to_string()))
    }
}

impl From<Vec<&str>> for Input<Vec<String>> {
    fn from(value: Vec<&str>) -> Self {
        Input::StaticValue(value.into_iter().map(|s| s.to_string()).collect())
    }
}

impl From<Vec<&str>> for Input<Option<Vec<String>>> {
    fn from(value: Vec<&str>) -> Self {
        Input::StaticValue(Some(value.into_iter().map(|s| s.to_string()).collect()))
    }
}

impl<T: Serialize, const N: usize> From<[T; N]> for Input<Vec<T>>
where
    T: Serialize,
{
    fn from(value: [T; N]) -> Self {
        Input::StaticValue(value.into_iter().collect())
    }
}

impl<T: Serialize, const N: usize> From<[T; N]> for Input<Option<Vec<T>>>
where
    T: Serialize,
{
    fn from(value: [T; N]) -> Self {
        Input::StaticValue(Some(value.into_iter().collect()))
    }
}

impl<const N: usize> From<[&str; N]> for Input<Vec<String>> {
    fn from(value: [&str; N]) -> Self {
        Input::StaticValue(value.into_iter().map(|s| s.to_string()).collect())
    }
}

impl<const N: usize> From<[&str; N]> for Input<Option<Vec<String>>> {
    fn from(value: [&str; N]) -> Self {
        Input::StaticValue(Some(value.into_iter().map(|s| s.to_string()).collect()))
    }
}
