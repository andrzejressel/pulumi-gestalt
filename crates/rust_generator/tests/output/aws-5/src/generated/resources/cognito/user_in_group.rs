/// Adds the specified user to the specified group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user_pool::create(
///         "example",
///         UserPoolArgs::builder()
///             .name("example")
///             .password_policy(
///                 UserPoolPasswordPolicy::builder()
///                     .minimumLength(6)
///                     .requireNumbers(false)
///                     .requireSymbols(false)
///                     .requireUppercase(false)
///                     .temporaryPasswordValidityDays(7)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleUser = user::create(
///         "exampleUser",
///         UserArgs::builder()
///             .user_pool_id("${example.id}")
///             .username("example")
///             .build_struct(),
///     );
///     let exampleUserGroup = user_group::create(
///         "exampleUserGroup",
///         UserGroupArgs::builder()
///             .name("example")
///             .user_pool_id("${example.id}")
///             .build_struct(),
///     );
///     let exampleUserInGroup = user_in_group::create(
///         "exampleUserInGroup",
///         UserInGroupArgs::builder()
///             .group_name("${exampleUserGroup.name}")
///             .user_pool_id("${example.id}")
///             .username("${exampleUser.username}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod user_in_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserInGroupArgs {
        /// The name of the group to which the user is to be added.
        #[builder(into)]
        pub group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The user pool ID of the user and group.
        #[builder(into)]
        pub user_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The username of the user to be added to the group.
        #[builder(into)]
        pub username: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserInGroupResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name of the group to which the user is to be added.
        pub group_name: pulumi_gestalt_rust::Output<String>,
        /// The user pool ID of the user and group.
        pub user_pool_id: pulumi_gestalt_rust::Output<String>,
        /// The username of the user to be added to the group.
        pub username: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserInGroupArgs,
    ) -> UserInGroupResult {
        let group_name_binding = args.group_name.get_output(context);
        let user_pool_id_binding = args.user_pool_id.get_output(context);
        let username_binding = args.username.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cognito/userInGroup:UserInGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "username".into(),
                    value: &username_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserInGroupResult {
            id: o.get_field("id"),
            group_name: o.get_field("groupName"),
            user_pool_id: o.get_field("userPoolId"),
            username: o.get_field("username"),
        }
    }
}
