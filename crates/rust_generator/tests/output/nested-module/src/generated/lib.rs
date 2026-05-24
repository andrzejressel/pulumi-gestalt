pub mod nested {
    pub mod module {
        include!("resources/nested/module/resource.rs");
    }
}
pub mod provider {
    include!("provider/provider.rs");
}
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::foo")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_FOO: [u8; 44] = *b"{\"version\":\"0.0.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "0.0.1".to_string()
}
pub fn package() -> pulumi_gestalt_rust::Package {
    pulumi_gestalt_rust::Package {
        name: "foo".to_string(),
        kind: "resource".to_string(),
        version: "0.0.1".to_string(),
        server: "".to_string(),
        checksums: std::collections::BTreeMap::new(),
        parameterization: None,
    }
}
