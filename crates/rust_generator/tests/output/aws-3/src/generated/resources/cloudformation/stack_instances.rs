/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = stack_instances::create(
///         "example",
///         StackInstancesArgs::builder()
///             .accounts(vec!["123456789012", "234567890123",])
///             .regions(vec!["us-east-1", "us-west-2",])
///             .stack_set_name("${exampleAwsCloudformationStackSet.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example IAM Setup in Target Account
///
/// ```yaml
/// resources:
///   aWSCloudFormationStackSetExecutionRole:
///     type: aws:iam:Role
///     name: AWSCloudFormationStackSetExecutionRole
///     properties:
///       assumeRolePolicy: ${aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy.json}
///       name: AWSCloudFormationStackSetExecutionRole
///   aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicyRolePolicy:
///     type: aws:iam:RolePolicy
///     name: AWSCloudFormationStackSetExecutionRole_MinimumExecutionPolicy
///     properties:
///       name: MinimumExecutionPolicy
///       policy: ${aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy.json}
///       role: ${aWSCloudFormationStackSetExecutionRole.name}
/// variables:
///   aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             effect: Allow
///             principals:
///               - identifiers:
///                   - ${aWSCloudFormationStackSetAdministrationRole.arn}
///                 type: AWS
///   # Documentation: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html
///   # Additional IAM permissions necessary depend on the resources defined in the StackSet template
///   aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - cloudformation:*
///               - s3:*
///               - sns:*
///             effect: Allow
///             resources:
///               - '*'
/// ```
///
/// ### Example Deployment across Organizations account
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = stack_instances::create(
///         "example",
///         StackInstancesArgs::builder()
///             .deployment_targets(
///                 StackInstancesDeploymentTargets::builder()
///                     .organizationalUnitIds(
///                         vec!["${exampleAwsOrganizationsOrganization.roots[0].id}",],
///                     )
///                     .build_struct(),
///             )
///             .regions(vec!["us-west-2", "us-east-1",])
///             .stack_set_name("${exampleAwsCloudformationStackSet.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Import CloudFormation stack instances that target OUs, using the stack set name, `call_as`, and "OU" separated by commas (`,`). For example:
///
/// Using `pulumi import`, import CloudFormation stack instances using the stack set name and `call_as` separated by commas (`,`). If you are importing a stack instance targeting OUs, see the example below. For example:
///
/// ```sh
/// $ pulumi import aws:cloudformation/stackInstances:StackInstances example example,SELF
/// ```
/// Using `pulumi import`, Import CloudFormation stack instances that target OUs, using the stack set name, `call_as`, and "OU" separated by commas (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:cloudformation/stackInstances:StackInstances example example,SELF,OU
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod stack_instances {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StackInstancesArgs {
        /// Accounts where you want to create stack instances in the specified `regions`. You can specify either `accounts` or `deployment_targets`, but not both.
        #[builder(into, default)]
        pub accounts: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
        #[builder(into, default)]
        pub call_as: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// AWS Organizations accounts for which to create stack instances in the `regions`. stack sets doesn't deploy stack instances to the organization management account, even if the organization management account is in your organization or in an OU in your organization. Drift detection is not possible for most of this argument. See deployment_targets below.
        #[builder(into, default)]
        pub deployment_targets: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudformation::StackInstancesDeploymentTargets>,
        >,
        /// Preferences for how AWS CloudFormation performs a stack set operation. See operation_preferences below.
        #[builder(into, default)]
        pub operation_preferences: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::cloudformation::StackInstancesOperationPreferences,
            >,
        >,
        /// Key-value map of input parameters to override from the stack set for these instances. This argument's drift detection is limited to the first account and region since each instance can have unique parameters.
        #[builder(into, default)]
        pub parameter_overrides: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Regions where you want to create stack instances in the specified `accounts`.
        #[builder(into, default)]
        pub regions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Whether to remove the stack instances from the stack set, but not delete the stacks. You can't reassociate a retained stack or add an existing, saved stack to a new stack set. To retain the stack, ensure `retain_stacks = true` has been successfully applied _before_ an apply that would destroy the resource. Defaults to `false`.
        #[builder(into, default)]
        pub retain_stacks: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the stack set.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub stack_set_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct StackInstancesResult {
        /// Accounts where you want to create stack instances in the specified `regions`. You can specify either `accounts` or `deployment_targets`, but not both.
        pub accounts: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
        pub call_as: pulumi_gestalt_rust::Output<Option<String>>,
        /// AWS Organizations accounts for which to create stack instances in the `regions`. stack sets doesn't deploy stack instances to the organization management account, even if the organization management account is in your organization or in an OU in your organization. Drift detection is not possible for most of this argument. See deployment_targets below.
        pub deployment_targets: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudformation::StackInstancesDeploymentTargets>,
        >,
        /// Preferences for how AWS CloudFormation performs a stack set operation. See operation_preferences below.
        pub operation_preferences: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::cloudformation::StackInstancesOperationPreferences,
            >,
        >,
        /// Key-value map of input parameters to override from the stack set for these instances. This argument's drift detection is limited to the first account and region since each instance can have unique parameters.
        pub parameter_overrides: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Regions where you want to create stack instances in the specified `accounts`.
        pub regions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Whether to remove the stack instances from the stack set, but not delete the stacks. You can't reassociate a retained stack or add an existing, saved stack to a new stack set. To retain the stack, ensure `retain_stacks = true` has been successfully applied _before_ an apply that would destroy the resource. Defaults to `false`.
        pub retain_stacks: pulumi_gestalt_rust::Output<Option<bool>>,
        /// List of stack instances created from an organizational unit deployment target. This may not always be set depending on whether CloudFormation returns summaries for your configuration. See `stack_instance_summaries`.
        pub stack_instance_summaries: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cloudformation::StackInstancesStackInstanceSummary>,
        >,
        /// Name or unique ID of the stack set that the stack instance is associated with.
        pub stack_set_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the stack set.
        ///
        /// The following arguments are optional:
        pub stack_set_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StackInstancesArgs,
    ) -> StackInstancesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let accounts_binding = args.accounts.get_output(context);
        let call_as_binding = args.call_as.get_output(context);
        let deployment_targets_binding = args.deployment_targets.get_output(context);
        let operation_preferences_binding = args
            .operation_preferences
            .get_output(context);
        let parameter_overrides_binding = args.parameter_overrides.get_output(context);
        let regions_binding = args.regions.get_output(context);
        let retain_stacks_binding = args.retain_stacks.get_output(context);
        let stack_set_name_binding = args.stack_set_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudformation/stackInstances:StackInstances".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accounts".into(),
                    value: &accounts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "callAs".into(),
                    value: &call_as_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentTargets".into(),
                    value: &deployment_targets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "operationPreferences".into(),
                    value: &operation_preferences_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameterOverrides".into(),
                    value: &parameter_overrides_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regions".into(),
                    value: &regions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retainStacks".into(),
                    value: &retain_stacks_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stackSetName".into(),
                    value: &stack_set_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        StackInstancesResult {
            accounts: o.get_field("accounts"),
            call_as: o.get_field("callAs"),
            deployment_targets: o.get_field("deploymentTargets"),
            operation_preferences: o.get_field("operationPreferences"),
            parameter_overrides: o.get_field("parameterOverrides"),
            regions: o.get_field("regions"),
            retain_stacks: o.get_field("retainStacks"),
            stack_instance_summaries: o.get_field("stackInstanceSummaries"),
            stack_set_id: o.get_field("stackSetId"),
            stack_set_name: o.get_field("stackSetName"),
        }
    }
}
