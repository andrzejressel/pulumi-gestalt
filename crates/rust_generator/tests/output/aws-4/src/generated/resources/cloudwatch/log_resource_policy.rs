/// Provides a resource to manage a CloudWatch log resource policy.
///
/// ## Example Usage
///
/// ### Elasticsearch Log Publishing
///
/// ```yaml
/// resources:
///   elasticsearch-log-publishing-policyLogResourcePolicy:
///     type: aws:cloudwatch:LogResourcePolicy
///     name: elasticsearch-log-publishing-policy
///     properties:
///       policyDocument: ${["elasticsearch-log-publishing-policy"].json}
///       policyName: elasticsearch-log-publishing-policy
/// variables:
///   elasticsearch-log-publishing-policy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - logs:CreateLogStream
///               - logs:PutLogEvents
///               - logs:PutLogEventsBatch
///             resources:
///               - arn:aws:logs:*
///             principals:
///               - identifiers:
///                   - es.amazonaws.com
///                 type: Service
/// ```
///
/// ### Route53 Query Logging
///
/// ```yaml
/// resources:
///   route53-query-logging-policyLogResourcePolicy:
///     type: aws:cloudwatch:LogResourcePolicy
///     name: route53-query-logging-policy
///     properties:
///       policyDocument: ${["route53-query-logging-policy"].json}
///       policyName: route53-query-logging-policy
/// variables:
///   route53-query-logging-policy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - logs:CreateLogStream
///               - logs:PutLogEvents
///             resources:
///               - arn:aws:logs:*:*:log-group:/aws/route53/*
///             principals:
///               - identifiers:
///                   - route53.amazonaws.com
///                 type: Service
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch log resource policies using the policy name. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logResourcePolicy:LogResourcePolicy MyPolicy MyPolicy
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod log_resource_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogResourcePolicyArgs {
        /// Details of the resource policy, including the identity of the principal that is enabled to put logs to this account. This is formatted as a JSON string. Maximum length of 5120 characters.
        #[builder(into)]
        pub policy_document: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the resource policy.
        #[builder(into)]
        pub policy_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LogResourcePolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Details of the resource policy, including the identity of the principal that is enabled to put logs to this account. This is formatted as a JSON string. Maximum length of 5120 characters.
        pub policy_document: pulumi_gestalt_rust::Output<String>,
        /// Name of the resource policy.
        pub policy_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogResourcePolicyArgs,
    ) -> LogResourcePolicyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogResourcePolicyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> LogResourcePolicyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogResourcePolicyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> LogResourcePolicyResult {
        let policy_document_binding = args.policy_document.get_output(ctx);
        let policy_name_binding = args.policy_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/logResourcePolicy:LogResourcePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyDocument".into(),
                    value: &policy_document_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyName".into(),
                    value: &policy_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        LogResourcePolicyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            policy_document: o.get_field("policyDocument"),
            policy_name: o.get_field("policyName"),
        }
    }
}
