/// The provider type for the random package. By default, resources use package-wide configuration
/// settings, however an explicit `Provider` instance may be created and passed during resource
/// construction to achieve fine-grained programmatic control over provider settings. See the
/// [documentation](https://www.pulumi.com/docs/reference/programming-model/#providers) for more information.
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
#[allow(dead_code)]
pub struct ProviderResult {}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports, dead_code)]
pub fn create(context: &pulumi_gestalt_rust::Context, name: &str) -> ProviderResult {
    let request = pulumi_gestalt_rust::RegisterResourceRequest {
        type_: "pulumi:providers:random".into(),
        name: name.to_string(),
        version: super::get_version(),
        object: &[],
    };
    let _o = context.register_resource(request);
    ProviderResult {}
}
