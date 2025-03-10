/// Provides a CloudWatch Logs destination policy resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   testDestination:
///     type: aws:cloudwatch:LogDestination
///     name: test_destination
///     properties:
///       name: test_destination
///       roleArn: ${iamForCloudwatch.arn}
///       targetArn: ${kinesisForCloudwatch.arn}
///   testDestinationPolicyLogDestinationPolicy:
///     type: aws:cloudwatch:LogDestinationPolicy
///     name: test_destination_policy
///     properties:
///       destinationName: ${testDestination.name}
///       accessPolicy: ${testDestinationPolicy.json}
/// variables:
///   testDestinationPolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '123456789012'
///             actions:
///               - logs:PutSubscriptionFilter
///             resources:
///               - ${testDestination.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Logs destination policies using the `destination_name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logDestinationPolicy:LogDestinationPolicy test_destination_policy test_destination
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod log_destination_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogDestinationPolicyArgs {
        /// The policy document. This is a JSON formatted string.
        #[builder(into)]
        pub access_policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A name for the subscription filter
        #[builder(into)]
        pub destination_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specify true if you are updating an existing destination policy to grant permission to an organization ID instead of granting permission to individual AWS accounts.
        #[builder(into, default)]
        pub force_update: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct LogDestinationPolicyResult {
        /// The policy document. This is a JSON formatted string.
        pub access_policy: pulumi_gestalt_rust::Output<String>,
        /// A name for the subscription filter
        pub destination_name: pulumi_gestalt_rust::Output<String>,
        /// Specify true if you are updating an existing destination policy to grant permission to an organization ID instead of granting permission to individual AWS accounts.
        pub force_update: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogDestinationPolicyArgs,
    ) -> LogDestinationPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_policy_binding = args.access_policy.get_output(context);
        let destination_name_binding = args.destination_name.get_output(context);
        let force_update_binding = args.force_update.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/logDestinationPolicy:LogDestinationPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessPolicy".into(),
                    value: &access_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationName".into(),
                    value: &destination_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceUpdate".into(),
                    value: &force_update_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LogDestinationPolicyResult {
            access_policy: o.get_field("accessPolicy"),
            destination_name: o.get_field("destinationName"),
            force_update: o.get_field("forceUpdate"),
        }
    }
}
