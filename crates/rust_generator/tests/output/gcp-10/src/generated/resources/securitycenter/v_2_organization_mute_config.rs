/// Mute Findings is a volume management feature in Security Command Center
/// that lets you manually or programmatically hide irrelevant findings,
/// and create filters to automatically silence existing and future
/// findings based on criteria you specify.
///
///
/// To get more information about OrganizationMuteConfig, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/rest/v2/organizations.muteConfigs)
///
/// ## Example Usage
///
/// ### Scc V2 Organization Mute Config Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = v_2_organization_mute_config::create(
///         "default",
///         V2OrganizationMuteConfigArgs::builder()
///             .description(
///                 "My custom Cloud Security Command Center Finding Organization mute Configuration",
///             )
///             .filter("severity = \"HIGH\"")
///             .location("global")
///             .mute_config_id("my-config")
///             .organization("123456789")
///             .type_("STATIC")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// OrganizationMuteConfig can be imported using any of these accepted formats:
///
/// * `organizations/{{organization}}/locations/{{location}}/muteConfigs/{{mute_config_id}}`
///
/// * `{{organization}}/{{location}}/{{mute_config_id}}`
///
/// When using the `pulumi import` command, OrganizationMuteConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2OrganizationMuteConfig:V2OrganizationMuteConfig default organizations/{{organization}}/locations/{{location}}/muteConfigs/{{mute_config_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2OrganizationMuteConfig:V2OrganizationMuteConfig default {{organization}}/{{location}}/{{mute_config_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod v_2_organization_mute_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2OrganizationMuteConfigArgs {
        /// A description of the mute config.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An expression that defines the filter to apply across create/update
        /// events of findings. While creating a filter string, be mindful of
        /// the scope in which the mute configuration is being created. E.g.,
        /// If a filter contains project = X but is created under the
        /// project = Y scope, it might not match any findings.
        #[builder(into)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<String>,
        /// location Id is provided by organization. If not provided, Use global as default.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique identifier provided by the client within the parent scope.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub mute_config_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The organization whose Cloud Security Command Center the Mute
        /// Config lives in.
        #[builder(into)]
        pub organization: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of the mute config.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct V2OrganizationMuteConfigResult {
        /// The time at which the mute config was created. This field is set by
        /// the server and will be ignored if provided on config creation.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A description of the mute config.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// An expression that defines the filter to apply across create/update
        /// events of findings. While creating a filter string, be mindful of
        /// the scope in which the mute configuration is being created. E.g.,
        /// If a filter contains project = X but is created under the
        /// project = Y scope, it might not match any findings.
        pub filter: pulumi_gestalt_rust::Output<String>,
        /// location Id is provided by organization. If not provided, Use global as default.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Email address of the user who last edited the mute config. This
        /// field is set by the server and will be ignored if provided on
        /// config creation or update.
        pub most_recent_editor: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier provided by the client within the parent scope.
        ///
        ///
        /// - - -
        pub mute_config_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the mute config. Its format is
        /// organizations/{organization}/locations/global/muteConfigs/{configId},
        /// folders/{folder}/locations/global/muteConfigs/{configId},
        /// or projects/{project}/locations/global/muteConfigs/{configId}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The organization whose Cloud Security Command Center the Mute
        /// Config lives in.
        pub organization: pulumi_gestalt_rust::Output<String>,
        /// The type of the mute config.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Output only. The most recent time at which the mute config was
        /// updated. This field is set by the server and will be ignored if
        /// provided on config creation or update.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: V2OrganizationMuteConfigArgs,
    ) -> V2OrganizationMuteConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let location_binding = args.location.get_output(context);
        let mute_config_id_binding = args.mute_config_id.get_output(context);
        let organization_binding = args.organization.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:securitycenter/v2OrganizationMuteConfig:V2OrganizationMuteConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "muteConfigId".into(),
                    value: &mute_config_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organization".into(),
                    value: &organization_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        V2OrganizationMuteConfigResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            filter: o.get_field("filter"),
            location: o.get_field("location"),
            most_recent_editor: o.get_field("mostRecentEditor"),
            mute_config_id: o.get_field("muteConfigId"),
            name: o.get_field("name"),
            organization: o.get_field("organization"),
            type_: o.get_field("type"),
            update_time: o.get_field("updateTime"),
        }
    }
}
