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
    pub helm_release_settings: pulumi_gestalt_rust::Input<
        Option<super::types::HelmReleaseSettings>,
    >,
}
#[allow(dead_code)]
pub struct ProviderResult {
    /// Pulumi URN is the stable logical identity of this provider resource in the Pulumi stack.
    pub urn: pulumi_gestalt_rust::Output<String>,
    /// Pulumi ID is the unique identifier assigned by the provider to this resource.
    pub id: pulumi_gestalt_rust::Output<String>,
    /// Pulumi Provider ID is the combination of URN and ID. It is used when creating a resource.
    pub provider_id: pulumi_gestalt_rust::Output<String>,
}
impl pulumi_gestalt_rust::Provider for ProviderResult {
    fn get_provider_id(&self) -> pulumi_gestalt_rust::Output<String> {
        self.provider_id.clone()
    }
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
    create_with_options(context, name, args, None)
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports, dead_code)]
pub fn create_with_options(
    context: &pulumi_gestalt_rust::Context,
    name: &str,
    args: ProviderArgs,
    options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
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
        options,
    };
    let o = context.register_resource(request);
    ProviderResult {
        urn: o.get_urn(),
        id: o.get_id(),
        provider_id: o.get_provider_id(),
    }
}
