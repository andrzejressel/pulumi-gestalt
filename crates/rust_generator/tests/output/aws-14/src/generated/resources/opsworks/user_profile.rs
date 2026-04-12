/// Provides an OpsWorks User Profile resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myProfile = user_profile::create(
///         "myProfile",
///         UserProfileArgs::builder()
///             .ssh_username("my_user")
///             .user_arn("${user.arn}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod user_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserProfileArgs {
        /// Whether users can specify their own SSH public key through the My Settings page
        #[builder(into, default)]
        pub allow_self_management: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The users public key
        #[builder(into, default)]
        pub ssh_public_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ssh username, with witch this user wants to log in
        #[builder(into)]
        pub ssh_username: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The user's IAM ARN
        #[builder(into)]
        pub user_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserProfileResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Whether users can specify their own SSH public key through the My Settings page
        pub allow_self_management: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The users public key
        pub ssh_public_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ssh username, with witch this user wants to log in
        pub ssh_username: pulumi_gestalt_rust::Output<String>,
        /// The user's IAM ARN
        pub user_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserProfileArgs,
    ) -> UserProfileResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserProfileArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> UserProfileResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserProfileArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> UserProfileResult {
        let allow_self_management_binding = args.allow_self_management.get_output(ctx);
        let ssh_public_key_binding = args.ssh_public_key.get_output(ctx);
        let ssh_username_binding = args.ssh_username.get_output(ctx);
        let user_arn_binding = args.user_arn.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:opsworks/userProfile:UserProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowSelfManagement".into(),
                    value: &allow_self_management_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sshPublicKey".into(),
                    value: &ssh_public_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sshUsername".into(),
                    value: &ssh_username_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userArn".into(),
                    value: &user_arn_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        UserProfileResult {
            id: o.get_id(),
            urn: o.get_urn(),
            allow_self_management: o.get_field("allowSelfManagement"),
            ssh_public_key: o.get_field("sshPublicKey"),
            ssh_username: o.get_field("sshUsername"),
            user_arn: o.get_field("userArn"),
        }
    }
}
