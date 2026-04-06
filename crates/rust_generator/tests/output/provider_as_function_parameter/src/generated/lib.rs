pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {
    pub mod providers {
        include!("functions/providers/random_terraform_config.rs");
    }
}
pub mod types {}
#[unsafe(link_section = "pulumi_gestalt_provider::random")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_RANDOM: [u8; 44] = *b"{\"version\":\"0.0.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "0.0.1".to_string()
}
