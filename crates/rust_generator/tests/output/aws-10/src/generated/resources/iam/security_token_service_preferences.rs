/// Provides an IAM Security Token Service Preferences resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = security_token_service_preferences::create(
///         "example",
///         SecurityTokenServicePreferencesArgs::builder()
///             .global_endpoint_token_version("v2Token")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod security_token_service_preferences {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityTokenServicePreferencesArgs {
        /// The version of the STS global endpoint token. Valid values: `v1Token`, `v2Token`.
        #[builder(into)]
        pub global_endpoint_token_version: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct SecurityTokenServicePreferencesResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The version of the STS global endpoint token. Valid values: `v1Token`, `v2Token`.
        pub global_endpoint_token_version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityTokenServicePreferencesArgs,
    ) -> SecurityTokenServicePreferencesResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityTokenServicePreferencesArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SecurityTokenServicePreferencesResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityTokenServicePreferencesArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SecurityTokenServicePreferencesResult {
        let global_endpoint_token_version_binding = args
            .global_endpoint_token_version
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/securityTokenServicePreferences:SecurityTokenServicePreferences"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalEndpointTokenVersion".into(),
                    value: &global_endpoint_token_version_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        SecurityTokenServicePreferencesResult {
            id: o.get_id(),
            urn: o.get_urn(),
            global_endpoint_token_version: o.get_field("globalEndpointTokenVersion"),
        }
    }
}
