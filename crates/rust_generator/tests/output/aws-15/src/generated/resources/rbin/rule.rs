/// Resource for managing an AWS RBin Rule.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:rbin:Rule
///     properties:
///       description: example_rule
///       resourceType: EBS_SNAPSHOT
///       resourceTags:
///         - resourceTagKey: tag_key
///           resourceTagValue: tag_value
///       retentionPeriod:
///         retentionPeriodValue: 10
///         retentionPeriodUnit: DAYS
///       tags:
///         test_tag_key: test_tag_value
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import RBin Rule using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:rbin/rule:Rule example examplerule
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RuleArgs {
        /// The retention rule description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Information about the retention rule lock configuration. See `lock_configuration` below.
        #[builder(into, default)]
        pub lock_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rbin::RuleLockConfiguration>,
        >,
        /// Specifies the resource tags to use to identify resources that are to be retained by a tag-level retention rule. See `resource_tags` below.
        #[builder(into, default)]
        pub resource_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::rbin::RuleResourceTag>>,
        >,
        /// The resource type to be retained by the retention rule. Valid values are `EBS_SNAPSHOT` and `EC2_IMAGE`.
        #[builder(into)]
        pub resource_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Information about the retention period for which the retention rule is to retain resources. See `retention_period` below.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub retention_period: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::rbin::RuleRetentionPeriod,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RuleResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The retention rule description.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Information about the retention rule lock configuration. See `lock_configuration` below.
        pub lock_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::rbin::RuleLockConfiguration>,
        >,
        /// (Timestamp) The date and time at which the unlock delay is set to expire. Only returned for retention rules that have been unlocked and that are still within the unlock delay period.
        pub lock_end_time: pulumi_gestalt_rust::Output<String>,
        /// (Optional) The lock state of the retention rules to list. Only retention rules with the specified lock state are returned. Valid values are `locked`, `pending_unlock`, `unlocked`.
        pub lock_state: pulumi_gestalt_rust::Output<String>,
        /// Specifies the resource tags to use to identify resources that are to be retained by a tag-level retention rule. See `resource_tags` below.
        pub resource_tags: pulumi_gestalt_rust::Output<
            Vec<super::super::types::rbin::RuleResourceTag>,
        >,
        /// The resource type to be retained by the retention rule. Valid values are `EBS_SNAPSHOT` and `EC2_IMAGE`.
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// Information about the retention period for which the retention rule is to retain resources. See `retention_period` below.
        ///
        /// The following arguments are optional:
        pub retention_period: pulumi_gestalt_rust::Output<
            super::super::types::rbin::RuleRetentionPeriod,
        >,
        /// (String) The state of the retention rule. Only retention rules that are in the `available` state retain resources. Valid values include `pending` and `available`.
        pub status: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
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
        args: RuleArgs,
    ) -> RuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let lock_configuration_binding = args.lock_configuration.get_output(context);
        let resource_tags_binding = args.resource_tags.get_output(context);
        let resource_type_binding = args.resource_type.get_output(context);
        let retention_period_binding = args.retention_period.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rbin/rule:Rule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lockConfiguration".into(),
                    value: &lock_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceTags".into(),
                    value: &resource_tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionPeriod".into(),
                    value: &retention_period_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RuleResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            lock_configuration: o.get_field("lockConfiguration"),
            lock_end_time: o.get_field("lockEndTime"),
            lock_state: o.get_field("lockState"),
            resource_tags: o.get_field("resourceTags"),
            resource_type: o.get_field("resourceType"),
            retention_period: o.get_field("retentionPeriod"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
