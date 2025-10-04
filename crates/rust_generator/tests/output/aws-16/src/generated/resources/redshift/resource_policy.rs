/// Creates a new Amazon Redshift Resource Policy.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:redshift:ResourcePolicy
///     properties:
///       resourceArn: ${exampleAwsRedshiftCluster.clusterNamespaceArn}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Effect: Allow
///               Principal:
///                 AWS: arn:aws:iam::12345678901:root
///               Action: redshift:CreateInboundIntegration
///               Resource: ${exampleAwsRedshiftCluster.clusterNamespaceArn}
///               Sid: ""
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Resource Policies using the `resource_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/resourcePolicy:ResourcePolicy example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourcePolicyArgs {
        /// The content of the resource policy being updated.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the account to create or update a resource policy for.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourcePolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The content of the resource policy being updated.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the account to create or update a resource policy for.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourcePolicyArgs,
    ) -> ResourcePolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_binding = args.policy.get_output(context);
        let resource_arn_binding = args.resource_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshift/resourcePolicy:ResourcePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourcePolicyResult {
            id: o.get_field("id"),
            policy: o.get_field("policy"),
            resource_arn: o.get_field("resourceArn"),
        }
    }
}
