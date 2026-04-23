/// Allows you to set a policy of an SQS Queue
/// while referencing ARN of the queue within the policy.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   q:
///     type: aws:sqs:Queue
///     properties:
///       name: examplequeue
///   testQueuePolicy:
///     type: aws:sqs:QueuePolicy
///     name: test
///     properties:
///       queueUrl: ${q.id}
///       policy: ${test.json}
/// variables:
///   test:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: First
///             effect: Allow
///             principals:
///               - type: '*'
///                 identifiers:
///                   - '*'
///             actions:
///               - sqs:SendMessage
///             resources:
///               - ${q.arn}
///             conditions:
///               - test: ArnEquals
///                 variable: aws:SourceArn
///                 values:
///                   - ${example.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SQS Queue Policies using the queue URL. For example:
///
/// ```sh
/// $ pulumi import aws:sqs/queuePolicy:QueuePolicy test https://queue.amazonaws.com/123456789012/myqueue
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod queue_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueuePolicyArgs {
        /// The JSON policy for the SQS queue.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::Input<String>,
        /// The URL of the SQS Queue to which to attach the policy
        #[builder(into)]
        pub queue_url: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct QueuePolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The JSON policy for the SQS queue.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The URL of the SQS Queue to which to attach the policy
        pub queue_url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: QueuePolicyArgs,
    ) -> QueuePolicyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: QueuePolicyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> QueuePolicyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: QueuePolicyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> QueuePolicyResult {
        let policy_binding = args.policy.get_output(ctx);
        let queue_url_binding = args.queue_url.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sqs/queuePolicy:QueuePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queueUrl".into(),
                    value: &queue_url_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        QueuePolicyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            policy: o.get_field("policy"),
            queue_url: o.get_field("queueUrl"),
        }
    }
}
