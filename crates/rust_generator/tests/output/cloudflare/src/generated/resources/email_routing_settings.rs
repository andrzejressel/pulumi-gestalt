/// Provides a resource for managing Email Routing settings.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   myZone:
///     type: cloudflare:EmailRoutingSettings
///     name: my_zone
///     properties:
///       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
///       enabled: 'true'
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod email_routing_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailRoutingSettingsArgs {
        /// State of the zone settings for Email Routing. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::Input<bool>,
        /// Flag to check if the user skipped the configuration wizard.
        #[builder(into, default)]
        pub skip_wizard: pulumi_gestalt_rust::Input<Option<bool>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct EmailRoutingSettingsResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The date and time the settings have been created.
        pub created: pulumi_gestalt_rust::Output<String>,
        /// State of the zone settings for Email Routing. **Modifying this attribute will force creation of a new resource.**
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The date and time the settings have been modified.
        pub modified: pulumi_gestalt_rust::Output<String>,
        /// Domain of your zone.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Flag to check if the user skipped the configuration wizard.
        pub skip_wizard: pulumi_gestalt_rust::Output<bool>,
        /// Show the state of your account, and the type or configuration error.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Email Routing settings identifier.
        pub tag: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EmailRoutingSettingsArgs,
    ) -> EmailRoutingSettingsResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EmailRoutingSettingsArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> EmailRoutingSettingsResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EmailRoutingSettingsArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> EmailRoutingSettingsResult {
        let enabled_binding = args.enabled.get_output(ctx);
        let skip_wizard_binding = args.skip_wizard.get_output(ctx);
        let zone_id_binding = args.zone_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/emailRoutingSettings:EmailRoutingSettings".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipWizard".into(),
                    value: &skip_wizard_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        EmailRoutingSettingsResult {
            id: o.get_id(),
            urn: o.get_urn(),
            created: o.get_field("created"),
            enabled: o.get_field("enabled"),
            modified: o.get_field("modified"),
            name: o.get_field("name"),
            skip_wizard: o.get_field("skipWizard"),
            status: o.get_field("status"),
            tag: o.get_field("tag"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
