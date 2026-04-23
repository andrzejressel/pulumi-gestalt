///
///
/// ## Import
///
/// Using `pulumi import`, import exclusive management of inline policy assignments using the `user_name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/userPoliciesExclusive:UserPoliciesExclusive example MyUser
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod user_policies_exclusive {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserPoliciesExclusiveArgs {
        /// A list of inline policy names to be assigned to the user. Policies attached to this user but not configured in this argument will be removed.
        #[builder(into)]
        pub policy_names: pulumi_gestalt_rust::Input<Vec<String>>,
        /// IAM user name.
        #[builder(into)]
        pub user_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct UserPoliciesExclusiveResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// A list of inline policy names to be assigned to the user. Policies attached to this user but not configured in this argument will be removed.
        pub policy_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// IAM user name.
        pub user_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserPoliciesExclusiveArgs,
    ) -> UserPoliciesExclusiveResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserPoliciesExclusiveArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> UserPoliciesExclusiveResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserPoliciesExclusiveArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> UserPoliciesExclusiveResult {
        let policy_names_binding = args.policy_names.get_output(ctx);
        let user_name_binding = args.user_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/userPoliciesExclusive:UserPoliciesExclusive".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyNames".into(),
                    value: &policy_names_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: &user_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        UserPoliciesExclusiveResult {
            id: o.get_id(),
            urn: o.get_urn(),
            policy_names: o.get_field("policyNames"),
            user_name: o.get_field("userName"),
        }
    }
}
