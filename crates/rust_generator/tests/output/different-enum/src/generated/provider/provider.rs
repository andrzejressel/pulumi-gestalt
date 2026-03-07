#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
#[allow(dead_code)]
pub struct ProviderResult {
    /// Pulumi URN is the stable logical identity of this provider resource in the Pulumi stack.
    pub urn: pulumi_gestalt_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports, dead_code)]
pub fn create(context: &pulumi_gestalt_rust::Context, name: &str) -> ProviderResult {
    let request = pulumi_gestalt_rust::RegisterResourceRequest {
        type_: "pulumi:providers:plant".into(),
        name: name.to_string(),
        version: super::get_version(),
        object: &[],
    };
    let o = context.register_resource(request);
    ProviderResult { urn: o.get_urn() }
}
