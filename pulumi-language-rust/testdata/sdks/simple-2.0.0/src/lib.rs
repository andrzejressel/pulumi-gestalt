include!("resources/resource.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::simple")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_SIMPLE: [u8; 44] = *b"{\"version\":\"2.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "2.0.0".to_string()
}
