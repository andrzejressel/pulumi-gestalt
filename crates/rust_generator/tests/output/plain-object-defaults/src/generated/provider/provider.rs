/// The provider type for the kubernetes package.
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
#[derive(pulumi_gestalt_rust::__private::bon::Builder)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProviderArgs {
    /// BETA FEATURE - Options to configure the Helm Release resource.
    #[builder(into, default)]
    pub helm_release_settings: pulumi_gestalt_rust::InputOrOutput<
        Option<super::types::HelmReleaseSettings>,
    >,
}
#[allow(dead_code)]
pub struct ProviderResult {
    /// Pulumi URN is the stable logical identity of this provider resource in the Pulumi stack.
    pub urn: pulumi_gestalt_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports, dead_code)]
pub fn create(
    context: &pulumi_gestalt_rust::Context,
    name: &str,
    args: ProviderArgs,
) -> ProviderResult {
    let helm_release_settings_binding = args.helm_release_settings.get_output(context);
    let request = pulumi_gestalt_rust::RegisterResourceRequest {
        type_: "pulumi:providers:example".into(),
        name: name.to_string(),
        version: super::get_version(),
        object: &[
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "helmReleaseSettings".into(),
                value: &helm_release_settings_binding.drop_type(),
            },
        ],
    };
    let o = context.register_resource(request);
    ProviderResult { urn: o.get_urn() }
}
