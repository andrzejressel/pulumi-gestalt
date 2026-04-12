/// Resource for managing QuickSight User
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user::create(
///         "example",
///         UserArgs::builder()
///             .email("author@example.com")
///             .iam_arn("arn:aws:iam::123456789012:user/Example")
///             .identity_type("IAM")
///             .namespace("foo")
///             .session_name("an-author")
///             .user_role("AUTHOR")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// You cannot import this resource.
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// The ID for the AWS account that the user is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The email address of the user that you want to register.
        #[builder(into)]
        pub email: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of the IAM user or role that you are registering with Amazon QuickSight.
        #[builder(into, default)]
        pub iam_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon QuickSight supports several ways of managing the identity of users. This parameter accepts either  `IAM` or `QUICKSIGHT`. If `IAM` is specified, the `iam_arn` must also be specified.
        #[builder(into)]
        pub identity_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Quicksight namespace to create the user in. Defaults to `default`.
        #[builder(into, default)]
        pub namespace: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the IAM session to use when assuming roles that can embed QuickSight dashboards. Only valid for registering users using an assumed IAM role. Additionally, if registering multiple users using the same IAM role, each user needs to have a unique session name.
        #[builder(into, default)]
        pub session_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon QuickSight user name that you want to create for the user you are registering. Only valid for registering a user with `identity_type` set to `QUICKSIGHT`.
        #[builder(into, default)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon QuickSight role of the user. The user role can be one of the following: `READER`, `AUTHOR`, `ADMIN`, `READER_PRO`, `AUTHOR_PRO` or `ADMIN_PRO`.
        #[builder(into)]
        pub user_role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the user
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ID for the AWS account that the user is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// The email address of the user that you want to register.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the IAM user or role that you are registering with Amazon QuickSight.
        pub iam_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amazon QuickSight supports several ways of managing the identity of users. This parameter accepts either  `IAM` or `QUICKSIGHT`. If `IAM` is specified, the `iam_arn` must also be specified.
        pub identity_type: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Quicksight namespace to create the user in. Defaults to `default`.
        pub namespace: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the IAM session to use when assuming roles that can embed QuickSight dashboards. Only valid for registering users using an assumed IAM role. Additionally, if registering multiple users using the same IAM role, each user needs to have a unique session name.
        pub session_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon QuickSight user name that you want to create for the user you are registering. Only valid for registering a user with `identity_type` set to `QUICKSIGHT`.
        pub user_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon QuickSight role of the user. The user role can be one of the following: `READER`, `AUTHOR`, `ADMIN`, `READER_PRO`, `AUTHOR_PRO` or `ADMIN_PRO`.
        pub user_role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserArgs,
    ) -> UserResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> UserResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> UserResult {
        let aws_account_id_binding = args.aws_account_id.get_output(ctx);
        let email_binding = args.email.get_output(ctx);
        let iam_arn_binding = args.iam_arn.get_output(ctx);
        let identity_type_binding = args.identity_type.get_output(ctx);
        let namespace_binding = args.namespace.get_output(ctx);
        let session_name_binding = args.session_name.get_output(ctx);
        let user_name_binding = args.user_name.get_output(ctx);
        let user_role_binding = args.user_role.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:quicksight/user:User".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "email".into(),
                    value: &email_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamArn".into(),
                    value: &iam_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityType".into(),
                    value: &identity_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sessionName".into(),
                    value: &session_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: &user_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userRole".into(),
                    value: &user_role_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        UserResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            aws_account_id: o.get_field("awsAccountId"),
            email: o.get_field("email"),
            iam_arn: o.get_field("iamArn"),
            identity_type: o.get_field("identityType"),
            namespace: o.get_field("namespace"),
            session_name: o.get_field("sessionName"),
            user_name: o.get_field("userName"),
            user_role: o.get_field("userRole"),
        }
    }
}
