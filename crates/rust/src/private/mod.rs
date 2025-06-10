pub mod output;

pub extern crate bon;
#[cfg(not(target_arch = "wasm32"))]
pub extern crate pulumi_gestalt_rust_adapter_native;
#[cfg(target_arch = "wasm32")]
pub extern crate pulumi_gestalt_rust_adapter_wasm;
pub extern crate pulumi_gestalt_serde_constant_string;
pub extern crate serde;
