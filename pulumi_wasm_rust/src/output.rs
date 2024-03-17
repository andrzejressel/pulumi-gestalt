use std::fmt::Debug;
use std::marker::PhantomData;
use crate::bindings::component::pulumi_wasm::output_interface;
use uuid::Uuid;
use log::{info};

pub struct Output<T> {
    phantom: PhantomData<T>,
    future: output_interface::Output,
}

impl <T: serde::Serialize> From<T> for Output<T> {
    fn from(value: T) -> Output<T> {
        Output::new(&value)
    }
}

impl<T> Output<T> {

    ///
    /// # Safety
    ///
    /// Returns handle to inner output representation. Only needed in provider glue code.
    pub unsafe fn get_inner(&self) -> &output_interface::Output {
        &self.future
    }

    ///
    /// # Safety
    ///
    /// Underlying output type must the same as `F` and this Output will take ownership of the handle.
    /// This means that the handle must not be deallocated by something else.
    pub unsafe fn new_from_handle<F: serde::Serialize>(handle: u32) -> Output<F> {
        let output = output_interface::Output::from_handle(handle);
        Output {
            phantom: PhantomData::<F>,
            future: output,
        }
    }

    pub fn new<F: serde::Serialize>(value: &F) -> Self {
        let binding = rmp_serde::to_vec(value).unwrap();
        let arg = binding.as_slice();
        let resource = output_interface::Output::new(arg);
        Output {
            phantom: PhantomData,
            future: resource,
        }
    }

    pub fn map<B, F>(&self, f: F) -> Output<B>
    where
        F: Fn(T) -> B + Send + 'static,
        T: serde::de::DeserializeOwned + Debug,
        B: serde::Serialize + Debug,
    {

        let f = move |arg: Vec<u8>| {
            let arg = arg.clone();
            let argument = rmp_serde::decode::from_slice(&arg)?;   // .map_err(|e| format!("{e}")).map_err(|e| anyhow!(e))?;
            info!("Argument: {:?}", argument);
            let result = f(argument);
            info!("Result: {:?}", result);
            let result = rmp_serde::to_vec(&result)?;
            Ok(result)
        };

        let uuid = Uuid::now_v7().to_string();
        let mut map = crate::HASHMAP.lock().unwrap();
        map.insert(uuid.clone(), Box::new(f));

        let new_output = self.future.map(uuid.as_str());

        Output {
            phantom: PhantomData,
            future: new_output,
        }
    }
}
