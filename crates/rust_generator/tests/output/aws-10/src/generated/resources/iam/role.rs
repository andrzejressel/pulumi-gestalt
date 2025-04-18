/// Provides an IAM role.
///
/// > **NOTE:** If policies are attached to the role via the `aws.iam.PolicyAttachment` resource and you are modifying the role `name` or `path`, the `force_detach_policies` argument must be set to `true` and applied before attempting the operation otherwise you will encounter a `DeleteConflict` error. The `aws.iam.RolePolicyAttachment` resource (recommended) does not have this requirement.
///
/// > **NOTE:** If you use this resource's `managed_policy_arns` argument or `inline_policy` configuration blocks, this resource will take over exclusive management of the role's respective policy types (e.g., both policy types if both arguments are used). These arguments are incompatible with other ways of managing a role's policies, such as `aws.iam.PolicyAttachment`, `aws.iam.RolePolicyAttachment`, and `aws.iam.RolePolicy`. If you attempt to manage a role's policies by multiple means, you will get resource cycling and/or errors.
///
/// > **NOTE:** We suggest using explicit JSON encoding or `aws.iam.getPolicyDocument` when assigning a value to `policy`. They seamlessly translate configuration to JSON, enabling you to maintain consistency within your configuration without the need for context switches. Also, you can sidestep potential complications arising from formatting discrepancies, whitespace inconsistencies, and other nuances inherent to JSON.
///
/// ## Example Usage
///
/// ### Basic Example
///
/// ```yaml
/// resources:
///   testRole:
///     type: aws:iam:Role
///     name: test_role
///     properties:
///       name: test_role
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Sid: ""
///               Principal:
///                 Service: ec2.amazonaws.com
///       tags:
///         tag-key: tag-value
/// ```
///
/// ### Example of Using Data Source for Assume Role Policy
///
/// ```yaml
/// resources:
///   instance:
///     type: aws:iam:Role
///     properties:
///       name: instance_role
///       path: /system/
///       assumeRolePolicy: ${instanceAssumeRolePolicy.json}
/// variables:
///   instanceAssumeRolePolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             principals:
///               - type: Service
///                 identifiers:
///                   - ec2.amazonaws.com
/// ```
///
/// ### Example of Exclusive Inline Policies
///
/// > The `inline_policy` argument is deprecated. Use the `aws.iam.RolePolicy` resource instead. If Pulumi should exclusively manage all inline policy associations (the current behavior of this argument), use the `aws.iam.RolePoliciesExclusive` resource as well.
///
/// This example creates an IAM role with two inline IAM policies. If someone adds another inline policy out-of-band, on the next apply, this provider will remove that policy. If someone deletes these policies out-of-band, this provider will recreate them.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iam:Role
///     properties:
///       name: yak_role
///       assumeRolePolicy: ${instanceAssumeRolePolicy.json}
///       inlinePolicies:
///         - name: my_inline_policy
///           policy:
///             fn::toJSON:
///               Version: 2012-10-17
///               Statement:
///                 - Action:
///                     - ec2:Describe*
///                   Effect: Allow
///                   Resource: '*'
///         - name: policy-8675309
///           policy: ${inlinePolicy.json}
/// variables:
///   inlinePolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - ec2:DescribeAccountAttributes
///             resources:
///               - '*'
/// ```
///
/// ### Example of Removing Inline Policies
///
/// > The `inline_policy` argument is deprecated. Use the `aws.iam.RolePolicy` resource instead. If Pulumi should exclusively manage all inline policy associations (the current behavior of this argument), use the `aws.iam.RolePoliciesExclusive` resource as well.
///
/// This example creates an IAM role with what appears to be empty IAM `inline_policy` argument instead of using `inline_policy` as a configuration block. The result is that if someone were to add an inline policy out-of-band, on the next apply, this provider will remove that policy.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = role::create(
///         "example",
///         RoleArgs::builder()
///             .assume_role_policy("${instanceAssumeRolePolicy.json}")
///             .inline_policies(vec![RoleInlinePolicy::builder().build_struct(),])
///             .name("yak_role")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example of Exclusive Managed Policies
///
/// > The `managed_policy_arns` argument is deprecated. Use the `aws.iam.RolePolicyAttachment` resource instead. If Pulumi should exclusively manage all managed policy attachments (the current behavior of this argument), use the `aws.iam.RolePolicyAttachmentsExclusive` resource as well.
///
/// This example creates an IAM role and attaches two managed IAM policies. If someone attaches another managed policy out-of-band, on the next apply, this provider will detach that policy. If someone detaches these policies out-of-band, this provider will attach them again.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iam:Role
///     properties:
///       name: yak_role
///       assumeRolePolicy: ${instanceAssumeRolePolicy.json}
///       managedPolicyArns:
///         - ${policyOne.arn}
///         - ${policyTwo.arn}
///   policyOne:
///     type: aws:iam:Policy
///     name: policy_one
///     properties:
///       name: policy-618033
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - ec2:Describe*
///               Effect: Allow
///               Resource: '*'
///   policyTwo:
///     type: aws:iam:Policy
///     name: policy_two
///     properties:
///       name: policy-381966
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - s3:ListAllMyBuckets
///                 - s3:ListBucket
///                 - s3:HeadBucket
///               Effect: Allow
///               Resource: '*'
/// ```
///
/// ### Example of Removing Managed Policies
///
/// > The `managed_policy_arns` argument is deprecated. Use the `aws.iam.RolePolicyAttachment` resource instead. If Pulumi should exclusively manage all managed policy attachments (the current behavior of this argument), use the `aws.iam.RolePolicyAttachmentsExclusive` resource as well.
///
/// This example creates an IAM role with an empty `managed_policy_arns` argument. If someone attaches a policy out-of-band, on the next apply, this provider will detach that policy.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = role::create(
///         "example",
///         RoleArgs::builder()
///             .assume_role_policy("${instanceAssumeRolePolicy.json}")
///             .managed_policy_arns(vec![])
///             .name("yak_role")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM Roles using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/role:Role developer developer_name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod role {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RoleArgs {
        /// Policy that grants an entity permission to assume the role.
        ///
        /// > **NOTE:** The `assume_role_policy` is very similar to but slightly different than a standard IAM policy and cannot use an `aws.iam.Policy` resource.  However, it _can_ use an `aws.iam.getPolicyDocument` data source. See the example above of how this works.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub assume_role_policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the role.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to force detaching any policies the role has before destroying it. Defaults to `false`.
        #[builder(into, default)]
        pub force_detach_policies: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration block defining an exclusive set of IAM inline policies associated with the IAM role. See below. If no blocks are configured, Pulumi will not manage any inline policies in this resource. Configuring one empty block (i.e., `inline_policy {}`) will cause Pulumi to remove _all_ inline policies added out of band on `apply`.
        #[builder(into, default)]
        pub inline_policies: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::iam::RoleInlinePolicy>>,
        >,
        /// Set of exclusive IAM managed policy ARNs to attach to the IAM role. If this attribute is not configured, Pulumi will ignore policy attachments to this resource. When configured, Pulumi will align the role's managed policy attachments with this set by attaching or detaching managed policies. Configuring an empty set (i.e., `managed_policy_arns = []`) will cause Pulumi to remove _all_ managed policy attachments.
        #[builder(into, default)]
        pub managed_policy_arns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Maximum session duration (in seconds) that you want to set for the specified role. If you do not specify a value for this setting, the default maximum of one hour is applied. This setting can have a value from 1 hour to 12 hours.
        #[builder(into, default)]
        pub max_session_duration: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Friendly name of the role. If omitted, the provider will assign a random, unique name. See [IAM Identifiers](https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) for more information.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique friendly name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Path to the role. See [IAM Identifiers](https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) for more information.
        #[builder(into, default)]
        pub path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the policy that is used to set the permissions boundary for the role.
        #[builder(into, default)]
        pub permissions_boundary: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of tags for the IAM role. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RoleResult {
        /// Amazon Resource Name (ARN) specifying the role.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Policy that grants an entity permission to assume the role.
        ///
        /// > **NOTE:** The `assume_role_policy` is very similar to but slightly different than a standard IAM policy and cannot use an `aws.iam.Policy` resource.  However, it _can_ use an `aws.iam.getPolicyDocument` data source. See the example above of how this works.
        ///
        /// The following arguments are optional:
        pub assume_role_policy: pulumi_gestalt_rust::Output<String>,
        /// Creation date of the IAM role.
        pub create_date: pulumi_gestalt_rust::Output<String>,
        /// Description of the role.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to force detaching any policies the role has before destroying it. Defaults to `false`.
        pub force_detach_policies: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration block defining an exclusive set of IAM inline policies associated with the IAM role. See below. If no blocks are configured, Pulumi will not manage any inline policies in this resource. Configuring one empty block (i.e., `inline_policy {}`) will cause Pulumi to remove _all_ inline policies added out of band on `apply`.
        pub inline_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::types::iam::RoleInlinePolicy>,
        >,
        /// Set of exclusive IAM managed policy ARNs to attach to the IAM role. If this attribute is not configured, Pulumi will ignore policy attachments to this resource. When configured, Pulumi will align the role's managed policy attachments with this set by attaching or detaching managed policies. Configuring an empty set (i.e., `managed_policy_arns = []`) will cause Pulumi to remove _all_ managed policy attachments.
        pub managed_policy_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Maximum session duration (in seconds) that you want to set for the specified role. If you do not specify a value for this setting, the default maximum of one hour is applied. This setting can have a value from 1 hour to 12 hours.
        pub max_session_duration: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Friendly name of the role. If omitted, the provider will assign a random, unique name. See [IAM Identifiers](https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) for more information.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique friendly name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// Path to the role. See [IAM Identifiers](https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) for more information.
        pub path: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the policy that is used to set the permissions boundary for the role.
        pub permissions_boundary: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value mapping of tags for the IAM role. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Stable and unique string identifying the role.
        pub unique_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RoleArgs,
    ) -> RoleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let assume_role_policy_binding = args.assume_role_policy.get_output(context);
        let description_binding = args.description.get_output(context);
        let force_detach_policies_binding = args
            .force_detach_policies
            .get_output(context);
        let inline_policies_binding = args.inline_policies.get_output(context);
        let managed_policy_arns_binding = args.managed_policy_arns.get_output(context);
        let max_session_duration_binding = args.max_session_duration.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let path_binding = args.path.get_output(context);
        let permissions_boundary_binding = args.permissions_boundary.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/role:Role".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assumeRolePolicy".into(),
                    value: &assume_role_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDetachPolicies".into(),
                    value: &force_detach_policies_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inlinePolicies".into(),
                    value: &inline_policies_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedPolicyArns".into(),
                    value: &managed_policy_arns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxSessionDuration".into(),
                    value: &max_session_duration_binding.drop_type(),
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
                    name: "path".into(),
                    value: &path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissionsBoundary".into(),
                    value: &permissions_boundary_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RoleResult {
            arn: o.get_field("arn"),
            assume_role_policy: o.get_field("assumeRolePolicy"),
            create_date: o.get_field("createDate"),
            description: o.get_field("description"),
            force_detach_policies: o.get_field("forceDetachPolicies"),
            inline_policies: o.get_field("inlinePolicies"),
            managed_policy_arns: o.get_field("managedPolicyArns"),
            max_session_duration: o.get_field("maxSessionDuration"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            path: o.get_field("path"),
            permissions_boundary: o.get_field("permissionsBoundary"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            unique_id: o.get_field("uniqueId"),
        }
    }
}
