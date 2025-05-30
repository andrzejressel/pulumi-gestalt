/// Provides a ApplicationInsights Application resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:applicationinsights:Application
///     properties:
///       resourceGroupName: ${exampleGroup.name}
///   exampleGroup:
///     type: aws:resourcegroups:Group
///     name: example
///     properties:
///       name: example
///       resourceQuery:
///         query:
///           fn::toJSON:
///             ResourceTypeFilters:
///               - AWS::EC2::Instance
///             TagFilters:
///               - Key: Stage
///                 Values:
///                   - Test
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ApplicationInsights Applications using the `resource_group_name`. For example:
///
/// ```sh
/// $ pulumi import aws:applicationinsights/application:Application some some-application
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// Indicates whether Application Insights automatically configures unmonitored resources in the resource group.
        #[builder(into, default)]
        pub auto_config_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configures all of the resources in the resource group by applying the recommended configurations.
        #[builder(into, default)]
        pub auto_create: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Indicates whether Application Insights can listen to CloudWatch events for the application resources, such as instance terminated, failed deployment, and others.
        #[builder(into, default)]
        pub cwe_monitor_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Application Insights can create applications based on a resource group or on an account. To create an account-based application using all of the resources in the account, set this parameter to `ACCOUNT_BASED`.
        #[builder(into, default)]
        pub grouping_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When set to `true`, creates opsItems for any problems detected on an application.
        #[builder(into, default)]
        pub ops_center_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// SNS topic provided to Application Insights that is associated to the created opsItem. Allows you to receive notifications for updates to the opsItem.
        #[builder(into, default)]
        pub ops_item_sns_topic_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource group.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// ARN of the Application.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether Application Insights automatically configures unmonitored resources in the resource group.
        pub auto_config_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configures all of the resources in the resource group by applying the recommended configurations.
        pub auto_create: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Indicates whether Application Insights can listen to CloudWatch events for the application resources, such as instance terminated, failed deployment, and others.
        pub cwe_monitor_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Application Insights can create applications based on a resource group or on an account. To create an account-based application using all of the resources in the account, set this parameter to `ACCOUNT_BASED`.
        pub grouping_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// When set to `true`, creates opsItems for any problems detected on an application.
        pub ops_center_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// SNS topic provided to Application Insights that is associated to the created opsItem. Allows you to receive notifications for updates to the opsItem.
        pub ops_item_sns_topic_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the resource group.
        ///
        /// The following arguments are optional:
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_config_enabled_binding = args.auto_config_enabled.get_output(context);
        let auto_create_binding = args.auto_create.get_output(context);
        let cwe_monitor_enabled_binding = args.cwe_monitor_enabled.get_output(context);
        let grouping_type_binding = args.grouping_type.get_output(context);
        let ops_center_enabled_binding = args.ops_center_enabled.get_output(context);
        let ops_item_sns_topic_arn_binding = args
            .ops_item_sns_topic_arn
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:applicationinsights/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoConfigEnabled".into(),
                    value: &auto_config_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoCreate".into(),
                    value: &auto_create_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cweMonitorEnabled".into(),
                    value: &cwe_monitor_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupingType".into(),
                    value: &grouping_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "opsCenterEnabled".into(),
                    value: &ops_center_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "opsItemSnsTopicArn".into(),
                    value: &ops_item_sns_topic_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationResult {
            arn: o.get_field("arn"),
            auto_config_enabled: o.get_field("autoConfigEnabled"),
            auto_create: o.get_field("autoCreate"),
            cwe_monitor_enabled: o.get_field("cweMonitorEnabled"),
            grouping_type: o.get_field("groupingType"),
            ops_center_enabled: o.get_field("opsCenterEnabled"),
            ops_item_sns_topic_arn: o.get_field("opsItemSnsTopicArn"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
