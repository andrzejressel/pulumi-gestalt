pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {
    include!("functions/func_with_secrets.rs");
}
pub mod types {}
#[unsafe(link_section = "pulumi_gestalt_provider::mypkg")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_MYPKG: [u8; 44] = *b"{\"version\":\"0.0.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "0.0.1".to_string()
}
