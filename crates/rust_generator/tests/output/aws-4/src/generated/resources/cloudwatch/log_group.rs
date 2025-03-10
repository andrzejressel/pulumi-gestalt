/// Provides a CloudWatch Log Group resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   yada:
///     type: aws:cloudwatch:LogGroup
///     properties:
///       name: Yada
///       tags:
///         Environment: production
///         Application: serviceA
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cloudwatch Log Groups using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logGroup:LogGroup test_group yada
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod log_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogGroupArgs {
        /// The ARN of the KMS Key to use when encrypting log data. Please note, after the AWS KMS CMK is disassociated from the log group,
        /// AWS CloudWatch Logs stops encrypting newly ingested data for the log group. All previously ingested data remains encrypted, and AWS CloudWatch Logs requires
        /// permissions for the CMK whenever the encrypted data is requested.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specified the log class of the log group. Possible values are: `STANDARD` or `INFREQUENT_ACCESS`.
        #[builder(into, default)]
        pub log_group_class: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the log group. If omitted, this provider will assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the number of days
        /// you want to retain log events in the specified log group.  Possible values are: 1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1096, 1827, 2192, 2557, 2922, 3288, 3653, and 0.
        /// If you select 0, the events in the log group are always retained and never expire.
        #[builder(into, default)]
        pub retention_in_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Set to true if you do not wish the log group (and any logs it may contain) to be deleted at destroy time, and instead just remove the log group from the state.
        #[builder(into, default)]
        pub skip_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LogGroupResult {
        /// The Amazon Resource Name (ARN) specifying the log group. Any `:*` suffix added by the API, denoting all CloudWatch Log Streams under the CloudWatch Log Group, is removed for greater compatibility with other AWS services that do not accept the suffix.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the KMS Key to use when encrypting log data. Please note, after the AWS KMS CMK is disassociated from the log group,
        /// AWS CloudWatch Logs stops encrypting newly ingested data for the log group. All previously ingested data remains encrypted, and AWS CloudWatch Logs requires
        /// permissions for the CMK whenever the encrypted data is requested.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specified the log class of the log group. Possible values are: `STANDARD` or `INFREQUENT_ACCESS`.
        pub log_group_class: pulumi_gestalt_rust::Output<String>,
        /// The name of the log group. If omitted, this provider will assign a random, unique name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// Specifies the number of days
        /// you want to retain log events in the specified log group.  Possible values are: 1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1096, 1827, 2192, 2557, 2922, 3288, 3653, and 0.
        /// If you select 0, the events in the log group are always retained and never expire.
        pub retention_in_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Set to true if you do not wish the log group (and any logs it may contain) to be deleted at destroy time, and instead just remove the log group from the state.
        pub skip_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: LogGroupArgs,
    ) -> LogGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let log_group_class_binding = args.log_group_class.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let retention_in_days_binding = args.retention_in_days.get_output(context);
        let skip_destroy_binding = args.skip_destroy.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/logGroup:LogGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logGroupClass".into(),
                    value: &log_group_class_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionInDays".into(),
                    value: &retention_in_days_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LogGroupResult {
            arn: o.get_field("arn"),
            kms_key_id: o.get_field("kmsKeyId"),
            log_group_class: o.get_field("logGroupClass"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            retention_in_days: o.get_field("retentionInDays"),
            skip_destroy: o.get_field("skipDestroy"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
