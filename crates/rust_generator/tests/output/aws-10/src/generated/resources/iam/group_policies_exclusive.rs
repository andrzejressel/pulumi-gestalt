///
///
/// ## Import
///
/// Using `pulumi import`, import exclusive management of inline policy assignments using the `group_name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/groupPoliciesExclusive:GroupPoliciesExclusive example MyGroup
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod group_policies_exclusive {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupPoliciesExclusiveArgs {
        /// IAM group name.
        #[builder(into)]
        pub group_name: pulumi_gestalt_rust::Input<String>,
        /// A list of inline policy names to be assigned to the group. Policies attached to this group but not configured in this argument will be removed.
        #[builder(into)]
        pub policy_names: pulumi_gestalt_rust::Input<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct GroupPoliciesExclusiveResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// IAM group name.
        pub group_name: pulumi_gestalt_rust::Output<String>,
        /// A list of inline policy names to be assigned to the group. Policies attached to this group but not configured in this argument will be removed.
        pub policy_names: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GroupPoliciesExclusiveArgs,
    ) -> GroupPoliciesExclusiveResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GroupPoliciesExclusiveArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> GroupPoliciesExclusiveResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GroupPoliciesExclusiveArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> GroupPoliciesExclusiveResult {
        let group_name_binding = args.group_name.get_output(ctx);
        let policy_names_binding = args.policy_names.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/groupPoliciesExclusive:GroupPoliciesExclusive".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyNames".into(),
                    value: &policy_names_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        GroupPoliciesExclusiveResult {
            id: o.get_id(),
            urn: o.get_urn(),
            group_name: o.get_field("groupName"),
            policy_names: o.get_field("policyNames"),
        }
    }
}
