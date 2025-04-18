/// Provides an SNS topic resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let userUpdates = topic::create(
///         "userUpdates",
///         TopicArgs::builder().name("user-updates-topic").build_struct(),
///     );
/// }
/// ```
///
/// ## Example with Delivery Policy
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let userUpdates = topic::create(
///         "userUpdates",
///         TopicArgs::builder()
///             .delivery_policy(
///                 "{\n  \"http\": {\n    \"defaultHealthyRetryPolicy\": {\n      \"minDelayTarget\": 20,\n      \"maxDelayTarget\": 20,\n      \"numRetries\": 3,\n      \"numMaxDelayRetries\": 0,\n      \"numNoDelayRetries\": 0,\n      \"numMinDelayRetries\": 0,\n      \"backoffFunction\": \"linear\"\n    },\n    \"disableSubscriptionOverrides\": false,\n    \"defaultThrottlePolicy\": {\n      \"maxReceivesPerSecond\": 1\n    }\n  }\n}",
///             )
///             .name("user-updates-topic")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Example with Server-side encryption (SSE)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let userUpdates = topic::create(
///         "userUpdates",
///         TopicArgs::builder()
///             .kms_master_key_id("alias/aws/sns")
///             .name("user-updates-topic")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Example with First-In-First-Out (FIFO)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let userUpdates = topic::create(
///         "userUpdates",
///         TopicArgs::builder()
///             .content_based_deduplication(true)
///             .fifo_topic(true)
///             .name("user-updates-topic.fifo")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Message Delivery Status Arguments
///
/// The `<endpoint>_success_feedback_role_arn` and `<endpoint>_failure_feedback_role_arn` arguments are used to give Amazon SNS write access to use CloudWatch Logs on your behalf. The `<endpoint>_success_feedback_sample_rate` argument is for specifying the sample rate percentage (0-100) of successfully delivered messages. After you configure the  `<endpoint>_failure_feedback_role_arn` argument, then all failed message deliveries generate CloudWatch Logs.
///
/// ## Import
///
/// Using `pulumi import`, import SNS Topics using the topic `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:sns/topic:Topic user_updates arn:aws:sns:us-west-2:123456789012:my-topic
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod topic {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicArgs {
        /// IAM role for failure feedback
        #[builder(into, default)]
        pub application_failure_feedback_role_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The IAM role permitted to receive success feedback for this topic
        #[builder(into, default)]
        pub application_success_feedback_role_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Percentage of success to sample
        #[builder(into, default)]
        pub application_success_feedback_sample_rate: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The message archive policy for FIFO topics. More details in the [AWS documentation](https://docs.aws.amazon.com/sns/latest/dg/message-archiving-and-replay-topic-owner.html).
        #[builder(into, default)]
        pub archive_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enables content-based deduplication for FIFO topics. For more information, see the [related documentation](https://docs.aws.amazon.com/sns/latest/dg/fifo-message-dedup.html)
        #[builder(into, default)]
        pub content_based_deduplication: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The SNS delivery policy. More details in the [AWS documentation](https://docs.aws.amazon.com/sns/latest/dg/DeliveryPolicies.html).
        #[builder(into, default)]
        pub delivery_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name for the topic
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean indicating whether or not to create a FIFO (first-in-first-out) topic. FIFO topics can't deliver messages to customer managed endpoints, such as email addresses, mobile apps, SMS, or HTTP(S) endpoints. These endpoint types aren't guaranteed to preserve strict message ordering. Default is `false`.
        #[builder(into, default)]
        pub fifo_topic: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// IAM role for failure feedback
        #[builder(into, default)]
        pub firehose_failure_feedback_role_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The IAM role permitted to receive success feedback for this topic
        #[builder(into, default)]
        pub firehose_success_feedback_role_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Percentage of success to sample
        #[builder(into, default)]
        pub firehose_success_feedback_sample_rate: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// IAM role for failure feedback
        #[builder(into, default)]
        pub http_failure_feedback_role_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The IAM role permitted to receive success feedback for this topic
        #[builder(into, default)]
        pub http_success_feedback_role_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Percentage of success to sample
        #[builder(into, default)]
        pub http_success_feedback_sample_rate: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The ID of an AWS-managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see [Key Terms](https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms)
        #[builder(into, default)]
        pub kms_master_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IAM role for failure feedback
        #[builder(into, default)]
        pub lambda_failure_feedback_role_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The IAM role permitted to receive success feedback for this topic
        #[builder(into, default)]
        pub lambda_success_feedback_role_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Percentage of success to sample
        #[builder(into, default)]
        pub lambda_success_feedback_sample_rate: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The name of the topic. Topic names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 256 characters long. For a FIFO (first-in-first-out) topic, the name must end with the `.fifo` suffix. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The fully-formed AWS policy as JSON.
        #[builder(into, default)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If `SignatureVersion` should be [1 (SHA1) or 2 (SHA256)](https://docs.aws.amazon.com/sns/latest/dg/sns-verify-signature-of-message.html). The signature version corresponds to the hashing algorithm used while creating the signature of the notifications, subscription confirmations, or unsubscribe confirmation messages sent by Amazon SNS.
        #[builder(into, default)]
        pub signature_version: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// IAM role for failure feedback
        #[builder(into, default)]
        pub sqs_failure_feedback_role_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The IAM role permitted to receive success feedback for this topic
        #[builder(into, default)]
        pub sqs_success_feedback_role_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Percentage of success to sample
        #[builder(into, default)]
        pub sqs_success_feedback_sample_rate: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Tracing mode of an Amazon SNS topic. Valid values: `"PassThrough"`, `"Active"`.
        #[builder(into, default)]
        pub tracing_config: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TopicResult {
        /// IAM role for failure feedback
        pub application_failure_feedback_role_arn: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The IAM role permitted to receive success feedback for this topic
        pub application_success_feedback_role_arn: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Percentage of success to sample
        pub application_success_feedback_sample_rate: pulumi_gestalt_rust::Output<
            Option<i32>,
        >,
        /// The message archive policy for FIFO topics. More details in the [AWS documentation](https://docs.aws.amazon.com/sns/latest/dg/message-archiving-and-replay-topic-owner.html).
        pub archive_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN of the SNS topic, as a more obvious property (clone of id)
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The oldest timestamp at which a FIFO topic subscriber can start a replay.
        pub beginning_archive_time: pulumi_gestalt_rust::Output<String>,
        /// Enables content-based deduplication for FIFO topics. For more information, see the [related documentation](https://docs.aws.amazon.com/sns/latest/dg/fifo-message-dedup.html)
        pub content_based_deduplication: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The SNS delivery policy. More details in the [AWS documentation](https://docs.aws.amazon.com/sns/latest/dg/DeliveryPolicies.html).
        pub delivery_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name for the topic
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Boolean indicating whether or not to create a FIFO (first-in-first-out) topic. FIFO topics can't deliver messages to customer managed endpoints, such as email addresses, mobile apps, SMS, or HTTP(S) endpoints. These endpoint types aren't guaranteed to preserve strict message ordering. Default is `false`.
        pub fifo_topic: pulumi_gestalt_rust::Output<Option<bool>>,
        /// IAM role for failure feedback
        pub firehose_failure_feedback_role_arn: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The IAM role permitted to receive success feedback for this topic
        pub firehose_success_feedback_role_arn: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Percentage of success to sample
        pub firehose_success_feedback_sample_rate: pulumi_gestalt_rust::Output<
            Option<i32>,
        >,
        /// IAM role for failure feedback
        pub http_failure_feedback_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IAM role permitted to receive success feedback for this topic
        pub http_success_feedback_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Percentage of success to sample
        pub http_success_feedback_sample_rate: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of an AWS-managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see [Key Terms](https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms)
        pub kms_master_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// IAM role for failure feedback
        pub lambda_failure_feedback_role_arn: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The IAM role permitted to receive success feedback for this topic
        pub lambda_success_feedback_role_arn: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Percentage of success to sample
        pub lambda_success_feedback_sample_rate: pulumi_gestalt_rust::Output<
            Option<i32>,
        >,
        /// The name of the topic. Topic names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 256 characters long. For a FIFO (first-in-first-out) topic, the name must end with the `.fifo` suffix. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The AWS Account ID of the SNS topic owner
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// The fully-formed AWS policy as JSON.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// If `SignatureVersion` should be [1 (SHA1) or 2 (SHA256)](https://docs.aws.amazon.com/sns/latest/dg/sns-verify-signature-of-message.html). The signature version corresponds to the hashing algorithm used while creating the signature of the notifications, subscription confirmations, or unsubscribe confirmation messages sent by Amazon SNS.
        pub signature_version: pulumi_gestalt_rust::Output<i32>,
        /// IAM role for failure feedback
        pub sqs_failure_feedback_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IAM role permitted to receive success feedback for this topic
        pub sqs_success_feedback_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Percentage of success to sample
        pub sqs_success_feedback_sample_rate: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Tracing mode of an Amazon SNS topic. Valid values: `"PassThrough"`, `"Active"`.
        pub tracing_config: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TopicArgs,
    ) -> TopicResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_failure_feedback_role_arn_binding = args
            .application_failure_feedback_role_arn
            .get_output(context);
        let application_success_feedback_role_arn_binding = args
            .application_success_feedback_role_arn
            .get_output(context);
        let application_success_feedback_sample_rate_binding = args
            .application_success_feedback_sample_rate
            .get_output(context);
        let archive_policy_binding = args.archive_policy.get_output(context);
        let content_based_deduplication_binding = args
            .content_based_deduplication
            .get_output(context);
        let delivery_policy_binding = args.delivery_policy.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let fifo_topic_binding = args.fifo_topic.get_output(context);
        let firehose_failure_feedback_role_arn_binding = args
            .firehose_failure_feedback_role_arn
            .get_output(context);
        let firehose_success_feedback_role_arn_binding = args
            .firehose_success_feedback_role_arn
            .get_output(context);
        let firehose_success_feedback_sample_rate_binding = args
            .firehose_success_feedback_sample_rate
            .get_output(context);
        let http_failure_feedback_role_arn_binding = args
            .http_failure_feedback_role_arn
            .get_output(context);
        let http_success_feedback_role_arn_binding = args
            .http_success_feedback_role_arn
            .get_output(context);
        let http_success_feedback_sample_rate_binding = args
            .http_success_feedback_sample_rate
            .get_output(context);
        let kms_master_key_id_binding = args.kms_master_key_id.get_output(context);
        let lambda_failure_feedback_role_arn_binding = args
            .lambda_failure_feedback_role_arn
            .get_output(context);
        let lambda_success_feedback_role_arn_binding = args
            .lambda_success_feedback_role_arn
            .get_output(context);
        let lambda_success_feedback_sample_rate_binding = args
            .lambda_success_feedback_sample_rate
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let signature_version_binding = args.signature_version.get_output(context);
        let sqs_failure_feedback_role_arn_binding = args
            .sqs_failure_feedback_role_arn
            .get_output(context);
        let sqs_success_feedback_role_arn_binding = args
            .sqs_success_feedback_role_arn
            .get_output(context);
        let sqs_success_feedback_sample_rate_binding = args
            .sqs_success_feedback_sample_rate
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tracing_config_binding = args.tracing_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sns/topic:Topic".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationFailureFeedbackRoleArn".into(),
                    value: &application_failure_feedback_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationSuccessFeedbackRoleArn".into(),
                    value: &application_success_feedback_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationSuccessFeedbackSampleRate".into(),
                    value: &application_success_feedback_sample_rate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "archivePolicy".into(),
                    value: &archive_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentBasedDeduplication".into(),
                    value: &content_based_deduplication_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deliveryPolicy".into(),
                    value: &delivery_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fifoTopic".into(),
                    value: &fifo_topic_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firehoseFailureFeedbackRoleArn".into(),
                    value: &firehose_failure_feedback_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firehoseSuccessFeedbackRoleArn".into(),
                    value: &firehose_success_feedback_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firehoseSuccessFeedbackSampleRate".into(),
                    value: &firehose_success_feedback_sample_rate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpFailureFeedbackRoleArn".into(),
                    value: &http_failure_feedback_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpSuccessFeedbackRoleArn".into(),
                    value: &http_success_feedback_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpSuccessFeedbackSampleRate".into(),
                    value: &http_success_feedback_sample_rate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsMasterKeyId".into(),
                    value: &kms_master_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lambdaFailureFeedbackRoleArn".into(),
                    value: &lambda_failure_feedback_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lambdaSuccessFeedbackRoleArn".into(),
                    value: &lambda_success_feedback_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lambdaSuccessFeedbackSampleRate".into(),
                    value: &lambda_success_feedback_sample_rate_binding.drop_type(),
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
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signatureVersion".into(),
                    value: &signature_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqsFailureFeedbackRoleArn".into(),
                    value: &sqs_failure_feedback_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqsSuccessFeedbackRoleArn".into(),
                    value: &sqs_success_feedback_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqsSuccessFeedbackSampleRate".into(),
                    value: &sqs_success_feedback_sample_rate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tracingConfig".into(),
                    value: &tracing_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TopicResult {
            application_failure_feedback_role_arn: o
                .get_field("applicationFailureFeedbackRoleArn"),
            application_success_feedback_role_arn: o
                .get_field("applicationSuccessFeedbackRoleArn"),
            application_success_feedback_sample_rate: o
                .get_field("applicationSuccessFeedbackSampleRate"),
            archive_policy: o.get_field("archivePolicy"),
            arn: o.get_field("arn"),
            beginning_archive_time: o.get_field("beginningArchiveTime"),
            content_based_deduplication: o.get_field("contentBasedDeduplication"),
            delivery_policy: o.get_field("deliveryPolicy"),
            display_name: o.get_field("displayName"),
            fifo_topic: o.get_field("fifoTopic"),
            firehose_failure_feedback_role_arn: o
                .get_field("firehoseFailureFeedbackRoleArn"),
            firehose_success_feedback_role_arn: o
                .get_field("firehoseSuccessFeedbackRoleArn"),
            firehose_success_feedback_sample_rate: o
                .get_field("firehoseSuccessFeedbackSampleRate"),
            http_failure_feedback_role_arn: o.get_field("httpFailureFeedbackRoleArn"),
            http_success_feedback_role_arn: o.get_field("httpSuccessFeedbackRoleArn"),
            http_success_feedback_sample_rate: o
                .get_field("httpSuccessFeedbackSampleRate"),
            kms_master_key_id: o.get_field("kmsMasterKeyId"),
            lambda_failure_feedback_role_arn: o
                .get_field("lambdaFailureFeedbackRoleArn"),
            lambda_success_feedback_role_arn: o
                .get_field("lambdaSuccessFeedbackRoleArn"),
            lambda_success_feedback_sample_rate: o
                .get_field("lambdaSuccessFeedbackSampleRate"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            owner: o.get_field("owner"),
            policy: o.get_field("policy"),
            signature_version: o.get_field("signatureVersion"),
            sqs_failure_feedback_role_arn: o.get_field("sqsFailureFeedbackRoleArn"),
            sqs_success_feedback_role_arn: o.get_field("sqsSuccessFeedbackRoleArn"),
            sqs_success_feedback_sample_rate: o
                .get_field("sqsSuccessFeedbackSampleRate"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tracing_config: o.get_field("tracingConfig"),
        }
    }
}
