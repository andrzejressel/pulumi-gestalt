use serde::Serialize;
use serde_json::Value;
use crate::Output;

pub trait IntoOutputAny {
    fn as_output(&self) -> Output<Value>;
}

impl<T: Serialize> IntoOutputAny for T {
    fn as_output(&self) -> Output<Value> {
        let value =
            serde_json::to_value(self).expect("Failed to serialize value while exporting output");
        Output::new(value)
    }
}

impl<T: Serialize + Clone + Send + Sync + 'static> IntoOutputAny for Output<T> {
    fn as_output(&self) -> Output<Value> {
        self.clone().map(|value| {
            serde_json::to_value(value)
                .expect("Failed to serialize output value while exporting output")
        })
    }
}
