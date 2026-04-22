/// Provides an SNS data protection topic policy resource
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:sns:Topic
///     properties:
///       name: example
///   exampleDataProtectionPolicy:
///     type: aws:sns:DataProtectionPolicy
///     name: example
///     properties:
///       arn: ${example.arn}
///       policy:
///         fn::toJSON:
///           Description: Example data protection policy
///           Name: __example_data_protection_policy
///           Statement:
///             - DataDirection: Inbound
///               DataIdentifier:
///                 - arn:aws:dataprotection::aws:data-identifier/EmailAddress
///               Operation:
///                 Deny: {}
///               Principal:
///                 - '*'
///               Sid: __deny_statement_11ba9d96
///           Version: 2021-06-01
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SNS Data Protection Topic Policy using the topic ARN. For example:
///
/// ```sh
/// $ pulumi import aws:sns/dataProtectionPolicy:DataProtectionPolicy example arn:aws:sns:us-west-2:123456789012:example
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod data_protection_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataProtectionPolicyArgs {
        /// The ARN of the SNS topic
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::Input<String>,
        /// The fully-formed AWS policy as JSON. For more information about building AWS IAM policy documents with this provider, see the AWS IAM Policy Document Guide.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct DataProtectionPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the SNS topic
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The fully-formed AWS policy as JSON. For more information about building AWS IAM policy documents with this provider, see the AWS IAM Policy Document Guide.
        pub policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataProtectionPolicyArgs,
    ) -> DataProtectionPolicyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataProtectionPolicyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DataProtectionPolicyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataProtectionPolicyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DataProtectionPolicyResult {
        let arn_binding = args.arn.get_output(ctx);
        let policy_binding = args.policy.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sns/dataProtectionPolicy:DataProtectionPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DataProtectionPolicyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            policy: o.get_field("policy"),
        }
    }
}
