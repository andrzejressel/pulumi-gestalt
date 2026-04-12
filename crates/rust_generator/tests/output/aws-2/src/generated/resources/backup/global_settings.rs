/// Provides an AWS Backup Global Settings resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:backup:GlobalSettings
///     properties:
///       globalSettings:
///         isCrossAccountBackupEnabled: 'true'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup Global Settings using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:backup/globalSettings:GlobalSettings example 123456789012
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod global_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlobalSettingsArgs {
        /// A list of resources along with the opt-in preferences for the account.
        #[builder(into)]
        pub global_settings: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
    }
    #[allow(dead_code)]
    pub struct GlobalSettingsResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// A list of resources along with the opt-in preferences for the account.
        pub global_settings: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GlobalSettingsArgs,
    ) -> GlobalSettingsResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GlobalSettingsArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> GlobalSettingsResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GlobalSettingsArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> GlobalSettingsResult {
        let global_settings_binding = args.global_settings.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:backup/globalSettings:GlobalSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalSettings".into(),
                    value: &global_settings_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        GlobalSettingsResult {
            id: o.get_id(),
            urn: o.get_urn(),
            global_settings: o.get_field("globalSettings"),
        }
    }
}
