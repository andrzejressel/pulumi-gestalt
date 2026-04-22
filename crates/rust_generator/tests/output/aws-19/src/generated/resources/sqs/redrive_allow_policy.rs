/// Provides a SQS Queue Redrive Allow Policy resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   src:
///     type: aws:sqs:Queue
///     properties:
///       name: srcqueue
///       redrivePolicy:
///         fn::toJSON:
///           deadLetterTargetArn: ${example.arn}
///           maxReceiveCount: 4
///   example:
///     type: aws:sqs:Queue
///     properties:
///       name: examplequeue
///   exampleRedriveAllowPolicy:
///     type: aws:sqs:RedriveAllowPolicy
///     name: example
///     properties:
///       queueUrl: ${example.id}
///       redriveAllowPolicy:
///         fn::toJSON:
///           redrivePermission: byQueue
///           sourceQueueArns:
///             - ${src.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SQS Queue Redrive Allow Policies using the queue URL. For example:
///
/// ```sh
/// $ pulumi import aws:sqs/redriveAllowPolicy:RedriveAllowPolicy test https://queue.amazonaws.com/123456789012/myqueue
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod redrive_allow_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RedriveAllowPolicyArgs {
        /// The URL of the SQS Queue to which to attach the policy
        #[builder(into)]
        pub queue_url: pulumi_gestalt_rust::Input<String>,
        /// The JSON redrive allow policy for the SQS queue. Learn more in the [Amazon SQS dead-letter queues documentation](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html).
        #[builder(into)]
        pub redrive_allow_policy: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct RedriveAllowPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The URL of the SQS Queue to which to attach the policy
        pub queue_url: pulumi_gestalt_rust::Output<String>,
        /// The JSON redrive allow policy for the SQS queue. Learn more in the [Amazon SQS dead-letter queues documentation](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html).
        pub redrive_allow_policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RedriveAllowPolicyArgs,
    ) -> RedriveAllowPolicyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RedriveAllowPolicyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> RedriveAllowPolicyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RedriveAllowPolicyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> RedriveAllowPolicyResult {
        let queue_url_binding = args.queue_url.get_output(ctx);
        let redrive_allow_policy_binding = args.redrive_allow_policy.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sqs/redriveAllowPolicy:RedriveAllowPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queueUrl".into(),
                    value: &queue_url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redriveAllowPolicy".into(),
                    value: &redrive_allow_policy_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        RedriveAllowPolicyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            queue_url: o.get_field("queueUrl"),
            redrive_allow_policy: o.get_field("redriveAllowPolicy"),
        }
    }
}
