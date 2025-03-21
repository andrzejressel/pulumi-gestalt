/// Configuration for discovery to scan resources for profile generation. Only one discovery configuration may exist per organization, folder, or project.
///
///
/// To get more information about DiscoveryConfig, see:
///
/// * [API documentation](https://cloud.google.com/dlp/docs/reference/rest/v2/projects.locations.discoveryConfigs)
/// * How-to Guides
///     * [Schedule inspection scan](https://cloud.google.com/dlp/docs/schedule-inspection-scan)
///
/// ## Example Usage
///
/// ## Import
///
/// DiscoveryConfig can be imported using any of these accepted formats:
///
/// * `{{parent}}/discoveryConfigs/{{name}}`
///
/// * `{{parent}}/{{name}}`
///
/// When using the `pulumi import` command, DiscoveryConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataloss/preventionDiscoveryConfig:PreventionDiscoveryConfig default {{parent}}/discoveryConfigs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataloss/preventionDiscoveryConfig:PreventionDiscoveryConfig default {{parent}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod prevention_discovery_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PreventionDiscoveryConfigArgs {
        /// Actions to execute at the completion of scanning
        /// Structure is documented below.
        #[builder(into, default)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::dataloss::PreventionDiscoveryConfigAction>>,
        >,
        /// Display Name (max 1000 Chars)
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Detection logic for profile generation
        #[builder(into, default)]
        pub inspect_templates: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Location to create the discovery config in.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub org_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataloss::PreventionDiscoveryConfigOrgConfig>,
        >,
        /// The parent of the discovery config in any of the following formats:
        /// * `projects/{{project}}/locations/{{location}}`
        /// * `organizations/{{organization_id}}/locations/{{location}}`
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. A status for this configuration
        /// Possible values are: `RUNNING`, `PAUSED`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Target to match against for determining what to scan and how frequently
        /// Structure is documented below.
        #[builder(into, default)]
        pub targets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::dataloss::PreventionDiscoveryConfigTarget>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PreventionDiscoveryConfigResult {
        /// Actions to execute at the completion of scanning
        /// Structure is documented below.
        pub actions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::dataloss::PreventionDiscoveryConfigAction>>,
        >,
        /// Output only. The creation timestamp of a DiscoveryConfig.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Display Name (max 1000 Chars)
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Output only. A stream of errors encountered when the config was activated. Repeated errors may result in the config automatically being paused. Output only field. Will return the last 100 errors. Whenever the config is modified this list will be cleared.
        /// Structure is documented below.
        pub errors: pulumi_gestalt_rust::Output<
            Vec<super::super::types::dataloss::PreventionDiscoveryConfigError>,
        >,
        /// Detection logic for profile generation
        pub inspect_templates: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Output only. The timestamp of the last time this config was executed
        pub last_run_time: pulumi_gestalt_rust::Output<String>,
        /// Location to create the discovery config in.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Unique resource name for the DiscoveryConfig, assigned by the service when the DiscoveryConfig is created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A nested object resource.
        /// Structure is documented below.
        pub org_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataloss::PreventionDiscoveryConfigOrgConfig>,
        >,
        /// The parent of the discovery config in any of the following formats:
        /// * `projects/{{project}}/locations/{{location}}`
        /// * `organizations/{{organization_id}}/locations/{{location}}`
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Required. A status for this configuration
        /// Possible values are: `RUNNING`, `PAUSED`.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// Target to match against for determining what to scan and how frequently
        /// Structure is documented below.
        pub targets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::dataloss::PreventionDiscoveryConfigTarget>>,
        >,
        /// Output only. The last update timestamp of a DiscoveryConfig.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PreventionDiscoveryConfigArgs,
    ) -> PreventionDiscoveryConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_binding = args.actions.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let inspect_templates_binding = args.inspect_templates.get_output(context);
        let location_binding = args.location.get_output(context);
        let org_config_binding = args.org_config.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let status_binding = args.status.get_output(context);
        let targets_binding = args.targets.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataloss/preventionDiscoveryConfig:PreventionDiscoveryConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actions".into(),
                    value: &actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inspectTemplates".into(),
                    value: &inspect_templates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgConfig".into(),
                    value: &org_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targets".into(),
                    value: &targets_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PreventionDiscoveryConfigResult {
            actions: o.get_field("actions"),
            create_time: o.get_field("createTime"),
            display_name: o.get_field("displayName"),
            errors: o.get_field("errors"),
            inspect_templates: o.get_field("inspectTemplates"),
            last_run_time: o.get_field("lastRunTime"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            org_config: o.get_field("orgConfig"),
            parent: o.get_field("parent"),
            status: o.get_field("status"),
            targets: o.get_field("targets"),
            update_time: o.get_field("updateTime"),
        }
    }
}
