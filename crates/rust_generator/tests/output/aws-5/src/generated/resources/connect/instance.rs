/// Provides an Amazon Connect instance resource. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)
///
/// !> **WARN:** Amazon Connect enforces a limit of [100 combined instance creation and deletions every 30 days](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html#feature-limits). For example, if you create 80 instances and delete 20 of them, you must wait 30 days to create or delete another instance. Use care when creating or deleting instances.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:connect:Instance
///     properties:
///       identityManagementType: CONNECT_MANAGED
///       inboundCallsEnabled: true
///       instanceAlias: friendly-name-connect
///       outboundCallsEnabled: true
///       tags:
///         hello: world
/// ```
///
///
/// ### With Existing Active Directory
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = instance::create(
///         "test",
///         InstanceArgs::builder()
///             .directory_id("${testAwsDirectoryServiceDirectory.id}")
///             .identity_management_type("EXISTING_DIRECTORY")
///             .inbound_calls_enabled(true)
///             .instance_alias("friendly-name-connect")
///             .outbound_calls_enabled(true)
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### With SAML
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = instance::create(
///         "test",
///         InstanceArgs::builder()
///             .identity_management_type("SAML")
///             .inbound_calls_enabled(true)
///             .instance_alias("friendly-name-connect")
///             .outbound_calls_enabled(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Connect instances using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:connect/instance:Instance example f1288a1f-6193-445a-b47e-af739b2
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// Specifies whether auto resolve best voices is enabled. Defaults to `true`.
        #[builder(into, default)]
        pub auto_resolve_best_voices_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies whether contact flow logs are enabled. Defaults to `false`.
        #[builder(into, default)]
        pub contact_flow_logs_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether contact lens is enabled. Defaults to `true`.
        #[builder(into, default)]
        pub contact_lens_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The identifier for the directory if identity_management_type is `EXISTING_DIRECTORY`.
        #[builder(into, default)]
        pub directory_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether early media for outbound calls is enabled . Defaults to `true` if outbound calls is enabled.
        #[builder(into, default)]
        pub early_media_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the identity management type attached to the instance. Allowed Values are: `SAML`, `CONNECT_MANAGED`, `EXISTING_DIRECTORY`.
        #[builder(into)]
        pub identity_management_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies whether inbound calls are enabled.
        #[builder(into)]
        pub inbound_calls_enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Specifies the name of the instance. Required if `directory_id` not specified.
        #[builder(into, default)]
        pub instance_alias: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether multi-party calls/conference is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub multi_party_conference_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies whether outbound calls are enabled.
        #[builder(into)]
        pub outbound_calls_enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Tags to apply to the Instance. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        /// <!-- * `use_custom_tts_voices` - (Optional) Whether use custom tts voices is enabled. Defaults to `false` -->
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// Amazon Resource Name (ARN) of the instance.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether auto resolve best voices is enabled. Defaults to `true`.
        pub auto_resolve_best_voices_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether contact flow logs are enabled. Defaults to `false`.
        pub contact_flow_logs_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether contact lens is enabled. Defaults to `true`.
        pub contact_lens_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// When the instance was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// The identifier for the directory if identity_management_type is `EXISTING_DIRECTORY`.
        pub directory_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether early media for outbound calls is enabled . Defaults to `true` if outbound calls is enabled.
        pub early_media_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the identity management type attached to the instance. Allowed Values are: `SAML`, `CONNECT_MANAGED`, `EXISTING_DIRECTORY`.
        pub identity_management_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether inbound calls are enabled.
        pub inbound_calls_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the name of the instance. Required if `directory_id` not specified.
        pub instance_alias: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether multi-party calls/conference is enabled. Defaults to `false`.
        pub multi_party_conference_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether outbound calls are enabled.
        pub outbound_calls_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The service role of the instance.
        pub service_role: pulumi_gestalt_rust::Output<String>,
        /// The state of the instance.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Tags to apply to the Instance. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        /// <!-- * `use_custom_tts_voices` - (Optional) Whether use custom tts voices is enabled. Defaults to `false` -->
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceArgs,
    ) -> InstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_resolve_best_voices_enabled_binding = args
            .auto_resolve_best_voices_enabled
            .get_output(context);
        let contact_flow_logs_enabled_binding = args
            .contact_flow_logs_enabled
            .get_output(context);
        let contact_lens_enabled_binding = args.contact_lens_enabled.get_output(context);
        let directory_id_binding = args.directory_id.get_output(context);
        let early_media_enabled_binding = args.early_media_enabled.get_output(context);
        let identity_management_type_binding = args
            .identity_management_type
            .get_output(context);
        let inbound_calls_enabled_binding = args
            .inbound_calls_enabled
            .get_output(context);
        let instance_alias_binding = args.instance_alias.get_output(context);
        let multi_party_conference_enabled_binding = args
            .multi_party_conference_enabled
            .get_output(context);
        let outbound_calls_enabled_binding = args
            .outbound_calls_enabled
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:connect/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoResolveBestVoicesEnabled".into(),
                    value: &auto_resolve_best_voices_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contactFlowLogsEnabled".into(),
                    value: &contact_flow_logs_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contactLensEnabled".into(),
                    value: &contact_lens_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "earlyMediaEnabled".into(),
                    value: &early_media_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityManagementType".into(),
                    value: &identity_management_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inboundCallsEnabled".into(),
                    value: &inbound_calls_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceAlias".into(),
                    value: &instance_alias_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiPartyConferenceEnabled".into(),
                    value: &multi_party_conference_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outboundCallsEnabled".into(),
                    value: &outbound_calls_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceResult {
            arn: o.get_field("arn"),
            auto_resolve_best_voices_enabled: o
                .get_field("autoResolveBestVoicesEnabled"),
            contact_flow_logs_enabled: o.get_field("contactFlowLogsEnabled"),
            contact_lens_enabled: o.get_field("contactLensEnabled"),
            created_time: o.get_field("createdTime"),
            directory_id: o.get_field("directoryId"),
            early_media_enabled: o.get_field("earlyMediaEnabled"),
            identity_management_type: o.get_field("identityManagementType"),
            inbound_calls_enabled: o.get_field("inboundCallsEnabled"),
            instance_alias: o.get_field("instanceAlias"),
            multi_party_conference_enabled: o.get_field("multiPartyConferenceEnabled"),
            outbound_calls_enabled: o.get_field("outboundCallsEnabled"),
            service_role: o.get_field("serviceRole"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
