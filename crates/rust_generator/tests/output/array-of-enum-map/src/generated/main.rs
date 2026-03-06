include!("resources/example_server.rs");
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod provider {
    #[allow(dead_code)]
    pub struct ProviderResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(context: &pulumi_gestalt_rust::Context, name: &str) -> ProviderResult {
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "pulumi:providers:example".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[],
        };
        let o = context.register_resource(request);
        ProviderResult {
            id: o.get_field("id"),
        }
    }
}
pub mod functions {}
pub mod types {
    include!("types/annotation_store_schema_value_type.rs");
}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::example")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_EXAMPLE: [u8; 44] = *b"{\"version\":\"1.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "1.0.0".to_string()
}
