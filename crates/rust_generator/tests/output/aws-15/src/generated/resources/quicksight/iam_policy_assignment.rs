/// Resource for managing an AWS QuickSight IAM Policy Assignment.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = iam_policy_assignment::create(
///         "example",
///         IamPolicyAssignmentArgs::builder()
///             .assignment_name("example")
///             .assignment_status("ENABLED")
///             .identities(
///                 IamPolicyAssignmentIdentities::builder()
///                     .users(vec!["${exampleAwsQuicksightUser.userName}",])
///                     .build_struct(),
///             )
///             .policy_arn("${exampleAwsIamPolicy.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import QuickSight IAM Policy Assignment using the AWS account ID, namespace, and assignment name separated by commas (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/iamPolicyAssignment:IamPolicyAssignment example 123456789012,default,example
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod iam_policy_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IamPolicyAssignmentArgs {
        /// Name of the assignment.
        #[builder(into)]
        pub assignment_name: pulumi_gestalt_rust::Input<String>,
        /// Status of the assignment. Valid values are `ENABLED`, `DISABLED`, and `DRAFT`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub assignment_status: pulumi_gestalt_rust::Input<String>,
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::Input<Option<String>>,
        /// Amazon QuickSight users, groups, or both to assign the policy to. See `identities` block.
        #[builder(into, default)]
        pub identities: pulumi_gestalt_rust::Input<
            Option<super::super::types::quicksight::IamPolicyAssignmentIdentities>,
        >,
        /// Namespace that contains the assignment. Defaults to `default`.
        #[builder(into, default)]
        pub namespace: pulumi_gestalt_rust::Input<Option<String>>,
        /// ARN of the IAM policy to apply to the Amazon QuickSight users and groups specified in this assignment.
        #[builder(into, default)]
        pub policy_arn: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IamPolicyAssignmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Assignment ID.
        pub assignment_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the assignment.
        pub assignment_name: pulumi_gestalt_rust::Output<String>,
        /// Status of the assignment. Valid values are `ENABLED`, `DISABLED`, and `DRAFT`.
        ///
        /// The following arguments are optional:
        pub assignment_status: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// Amazon QuickSight users, groups, or both to assign the policy to. See `identities` block.
        pub identities: pulumi_gestalt_rust::Output<
            Option<super::super::types::quicksight::IamPolicyAssignmentIdentities>,
        >,
        /// Namespace that contains the assignment. Defaults to `default`.
        pub namespace: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM policy to apply to the Amazon QuickSight users and groups specified in this assignment.
        pub policy_arn: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IamPolicyAssignmentArgs,
    ) -> IamPolicyAssignmentResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IamPolicyAssignmentArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> IamPolicyAssignmentResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IamPolicyAssignmentArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> IamPolicyAssignmentResult {
        let assignment_name_binding = args.assignment_name.get_output(ctx);
        let assignment_status_binding = args.assignment_status.get_output(ctx);
        let aws_account_id_binding = args.aws_account_id.get_output(ctx);
        let identities_binding = args.identities.get_output(ctx);
        let namespace_binding = args.namespace.get_output(ctx);
        let policy_arn_binding = args.policy_arn.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:quicksight/iamPolicyAssignment:IamPolicyAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assignmentName".into(),
                    value: &assignment_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assignmentStatus".into(),
                    value: &assignment_status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identities".into(),
                    value: &identities_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyArn".into(),
                    value: &policy_arn_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        IamPolicyAssignmentResult {
            id: o.get_id(),
            urn: o.get_urn(),
            assignment_id: o.get_field("assignmentId"),
            assignment_name: o.get_field("assignmentName"),
            assignment_status: o.get_field("assignmentStatus"),
            aws_account_id: o.get_field("awsAccountId"),
            identities: o.get_field("identities"),
            namespace: o.get_field("namespace"),
            policy_arn: o.get_field("policyArn"),
        }
    }
}
