/// Provides a DMS Serverless replication config resource.
///
/// > **NOTE:** Changing most arguments will stop the replication if it is running. You can set `start_replication` to resume the replication afterwards.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   name:
///     type: aws:dms:ReplicationConfig
///     properties:
///       replicationConfigIdentifier: test-dms-serverless-replication-tf
///       resourceIdentifier: test-dms-serverless-replication-tf
///       replicationType: cdc
///       sourceEndpointArn: ${source.endpointArn}
///       targetEndpointArn: ${target.endpointArn}
///       tableMappings: |2
///           {
///             "rules":[{"rule-type":"selection","rule-id":"1","rule-name":"1","rule-action":"include","object-locator":{"schema-name":"%%","table-name":"%%"}}]
///           }
///       startReplication: true
///       computeConfig:
///         replicationSubnetGroupId: ${default.replicationSubnetGroupId}
///         maxCapacityUnits: '64'
///         minCapacityUnits: '2'
///         preferredMaintenanceWindow: sun:23:45-mon:00:30
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a replication config using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:dms/replicationConfig:ReplicationConfig example arn:aws:dms:us-east-1:123456789012:replication-config:UX6OL6MHMMJKFFOXE3H7LLJCMEKBDUG4ZV7DRSI
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod replication_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationConfigArgs {
        /// Configuration block for provisioning an DMS Serverless replication.
        #[builder(into)]
        pub compute_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::dms::ReplicationConfigComputeConfig,
        >,
        /// Unique identifier that you want to use to create the config.
        #[builder(into)]
        pub replication_config_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An escaped JSON string that are used to provision this replication configuration. For example, [Change processing tuning settings](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TaskSettings.ChangeProcessingTuning.html)
        #[builder(into, default)]
        pub replication_settings: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The migration type. Can be one of `full-load | cdc | full-load-and-cdc`.
        #[builder(into)]
        pub replication_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Unique value or name that you set for a given resource that can be used to construct an Amazon Resource Name (ARN) for that resource. For more information, see [Fine-grained access control using resource names and tags](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#CHAP_Security.FineGrainedAccess)
        #[builder(into, default)]
        pub resource_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) string that uniquely identifies the source endpoint.
        #[builder(into)]
        pub source_endpoint_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to run or stop the serverless replication, default is false.
        #[builder(into, default)]
        pub start_replication: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// JSON settings for specifying supplemental data. For more information see [Specifying supplemental data for task settings](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.TaskData.html)
        #[builder(into, default)]
        pub supplemental_settings: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An escaped JSON string that contains the table mappings. For information on table mapping see [Using Table Mapping with an AWS Database Migration Service Task to Select and Filter Data](http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TableMapping.html)
        #[builder(into)]
        pub table_mappings: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Amazon Resource Name (ARN) string that uniquely identifies the target endpoint.
        #[builder(into)]
        pub target_endpoint_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ReplicationConfigResult {
        /// The Amazon Resource Name (ARN) for the serverless replication config.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for provisioning an DMS Serverless replication.
        pub compute_config: pulumi_gestalt_rust::Output<
            super::super::types::dms::ReplicationConfigComputeConfig,
        >,
        /// Unique identifier that you want to use to create the config.
        pub replication_config_identifier: pulumi_gestalt_rust::Output<String>,
        /// An escaped JSON string that are used to provision this replication configuration. For example, [Change processing tuning settings](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TaskSettings.ChangeProcessingTuning.html)
        pub replication_settings: pulumi_gestalt_rust::Output<String>,
        /// The migration type. Can be one of `full-load | cdc | full-load-and-cdc`.
        pub replication_type: pulumi_gestalt_rust::Output<String>,
        /// Unique value or name that you set for a given resource that can be used to construct an Amazon Resource Name (ARN) for that resource. For more information, see [Fine-grained access control using resource names and tags](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#CHAP_Security.FineGrainedAccess)
        pub resource_identifier: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) string that uniquely identifies the source endpoint.
        pub source_endpoint_arn: pulumi_gestalt_rust::Output<String>,
        /// Whether to run or stop the serverless replication, default is false.
        pub start_replication: pulumi_gestalt_rust::Output<Option<bool>>,
        /// JSON settings for specifying supplemental data. For more information see [Specifying supplemental data for task settings](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.TaskData.html)
        pub supplemental_settings: pulumi_gestalt_rust::Output<Option<String>>,
        /// An escaped JSON string that contains the table mappings. For information on table mapping see [Using Table Mapping with an AWS Database Migration Service Task to Select and Filter Data](http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TableMapping.html)
        pub table_mappings: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Amazon Resource Name (ARN) string that uniquely identifies the target endpoint.
        pub target_endpoint_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReplicationConfigArgs,
    ) -> ReplicationConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let compute_config_binding = args.compute_config.get_output(context);
        let replication_config_identifier_binding = args
            .replication_config_identifier
            .get_output(context);
        let replication_settings_binding = args.replication_settings.get_output(context);
        let replication_type_binding = args.replication_type.get_output(context);
        let resource_identifier_binding = args.resource_identifier.get_output(context);
        let source_endpoint_arn_binding = args.source_endpoint_arn.get_output(context);
        let start_replication_binding = args.start_replication.get_output(context);
        let supplemental_settings_binding = args
            .supplemental_settings
            .get_output(context);
        let table_mappings_binding = args.table_mappings.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_endpoint_arn_binding = args.target_endpoint_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:dms/replicationConfig:ReplicationConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computeConfig".into(),
                    value: &compute_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationConfigIdentifier".into(),
                    value: &replication_config_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationSettings".into(),
                    value: &replication_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationType".into(),
                    value: &replication_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceIdentifier".into(),
                    value: &resource_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceEndpointArn".into(),
                    value: &source_endpoint_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startReplication".into(),
                    value: &start_replication_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "supplementalSettings".into(),
                    value: &supplemental_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableMappings".into(),
                    value: &table_mappings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetEndpointArn".into(),
                    value: &target_endpoint_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReplicationConfigResult {
            arn: o.get_field("arn"),
            compute_config: o.get_field("computeConfig"),
            replication_config_identifier: o.get_field("replicationConfigIdentifier"),
            replication_settings: o.get_field("replicationSettings"),
            replication_type: o.get_field("replicationType"),
            resource_identifier: o.get_field("resourceIdentifier"),
            source_endpoint_arn: o.get_field("sourceEndpointArn"),
            start_replication: o.get_field("startReplication"),
            supplemental_settings: o.get_field("supplementalSettings"),
            table_mappings: o.get_field("tableMappings"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            target_endpoint_arn: o.get_field("targetEndpointArn"),
        }
    }
}
