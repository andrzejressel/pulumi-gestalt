include!("resources/resource.rs");
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {
    include!("types/data.rs");
    include!("types/inner_data.rs");
}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::ref-ref")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_REF_REF: [u8; 45] = *b"{\"version\":\"12.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "12.0.0".to_string()
}
pub fn package() -> pulumi_gestalt_rust::Package {
    pulumi_gestalt_rust::Package {
        name: "ref-ref".to_string(),
        kind: "resource".to_string(),
        version: "12.0.0".to_string(),
        server: "".to_string(),
        checksums: std::collections::BTreeMap::new(),
        parameterization: None,
    }
}
