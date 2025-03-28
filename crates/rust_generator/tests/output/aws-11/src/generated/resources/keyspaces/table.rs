/// Provides a Keyspaces Table.
///
/// More information about Keyspaces tables can be found in the [Keyspaces Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/working-with-tables.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = table::create(
///         "example",
///         TableArgs::builder()
///             .keyspace_name("${exampleAwsKeyspacesKeyspace.name}")
///             .schema_definition(
///                 TableSchemaDefinition::builder()
///                     .columns(
///                         vec![
///                             TableSchemaDefinitionColumn::builder().name("Message"). type
///                             ("ASCII").build_struct(),
///                         ],
///                     )
///                     .partitionKeys(
///                         vec![
///                             TableSchemaDefinitionPartitionKey::builder().name("Message")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .table_name("my_table")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a table using the `keyspace_name` and `table_name` separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:keyspaces/table:Table example my_keyspace/my_table
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableArgs {
        /// Specifies the read/write throughput capacity mode for the table.
        #[builder(into, default)]
        pub capacity_specification: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::keyspaces::TableCapacitySpecification>,
        >,
        /// Enables client-side timestamps for the table. By default, the setting is disabled.
        #[builder(into, default)]
        pub client_side_timestamps: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::keyspaces::TableClientSideTimestamps>,
        >,
        /// A description of the table.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::keyspaces::TableComment>,
        >,
        /// The default Time to Live setting in seconds for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL-how-it-works.html#ttl-howitworks_default_ttl).
        #[builder(into, default)]
        pub default_time_to_live: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies how the encryption key for encryption at rest is managed for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/EncryptionAtRest.html).
        #[builder(into, default)]
        pub encryption_specification: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::keyspaces::TableEncryptionSpecification>,
        >,
        /// The name of the keyspace that the table is going to be created in.
        #[builder(into)]
        pub keyspace_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies if point-in-time recovery is enabled or disabled for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/PointInTimeRecovery.html).
        #[builder(into, default)]
        pub point_in_time_recovery: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::keyspaces::TablePointInTimeRecovery>,
        >,
        /// Describes the schema of the table.
        #[builder(into)]
        pub schema_definition: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::keyspaces::TableSchemaDefinition,
        >,
        /// The name of the table.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub table_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Enables Time to Live custom settings for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL.html).
        #[builder(into, default)]
        pub ttl: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::keyspaces::TableTtl>,
        >,
    }
    #[allow(dead_code)]
    pub struct TableResult {
        /// The ARN of the table.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the read/write throughput capacity mode for the table.
        pub capacity_specification: pulumi_gestalt_rust::Output<
            super::super::types::keyspaces::TableCapacitySpecification,
        >,
        /// Enables client-side timestamps for the table. By default, the setting is disabled.
        pub client_side_timestamps: pulumi_gestalt_rust::Output<
            Option<super::super::types::keyspaces::TableClientSideTimestamps>,
        >,
        /// A description of the table.
        pub comment: pulumi_gestalt_rust::Output<
            super::super::types::keyspaces::TableComment,
        >,
        /// The default Time to Live setting in seconds for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL-how-it-works.html#ttl-howitworks_default_ttl).
        pub default_time_to_live: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies how the encryption key for encryption at rest is managed for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/EncryptionAtRest.html).
        pub encryption_specification: pulumi_gestalt_rust::Output<
            super::super::types::keyspaces::TableEncryptionSpecification,
        >,
        /// The name of the keyspace that the table is going to be created in.
        pub keyspace_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies if point-in-time recovery is enabled or disabled for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/PointInTimeRecovery.html).
        pub point_in_time_recovery: pulumi_gestalt_rust::Output<
            super::super::types::keyspaces::TablePointInTimeRecovery,
        >,
        /// Describes the schema of the table.
        pub schema_definition: pulumi_gestalt_rust::Output<
            super::super::types::keyspaces::TableSchemaDefinition,
        >,
        /// The name of the table.
        ///
        /// The following arguments are optional:
        pub table_name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enables Time to Live custom settings for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL.html).
        pub ttl: pulumi_gestalt_rust::Output<
            Option<super::super::types::keyspaces::TableTtl>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TableArgs,
    ) -> TableResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let capacity_specification_binding = args
            .capacity_specification
            .get_output(context);
        let client_side_timestamps_binding = args
            .client_side_timestamps
            .get_output(context);
        let comment_binding = args.comment.get_output(context);
        let default_time_to_live_binding = args.default_time_to_live.get_output(context);
        let encryption_specification_binding = args
            .encryption_specification
            .get_output(context);
        let keyspace_name_binding = args.keyspace_name.get_output(context);
        let point_in_time_recovery_binding = args
            .point_in_time_recovery
            .get_output(context);
        let schema_definition_binding = args.schema_definition.get_output(context);
        let table_name_binding = args.table_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let ttl_binding = args.ttl.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:keyspaces/table:Table".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacitySpecification".into(),
                    value: &capacity_specification_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientSideTimestamps".into(),
                    value: &client_side_timestamps_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: &comment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultTimeToLive".into(),
                    value: &default_time_to_live_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionSpecification".into(),
                    value: &encryption_specification_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyspaceName".into(),
                    value: &keyspace_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pointInTimeRecovery".into(),
                    value: &point_in_time_recovery_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schemaDefinition".into(),
                    value: &schema_definition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ttl".into(),
                    value: &ttl_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TableResult {
            arn: o.get_field("arn"),
            capacity_specification: o.get_field("capacitySpecification"),
            client_side_timestamps: o.get_field("clientSideTimestamps"),
            comment: o.get_field("comment"),
            default_time_to_live: o.get_field("defaultTimeToLive"),
            encryption_specification: o.get_field("encryptionSpecification"),
            keyspace_name: o.get_field("keyspaceName"),
            point_in_time_recovery: o.get_field("pointInTimeRecovery"),
            schema_definition: o.get_field("schemaDefinition"),
            table_name: o.get_field("tableName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            ttl: o.get_field("ttl"),
        }
    }
}
