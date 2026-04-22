/// Resource for managing an AWS DataZone User Profile.
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
///     let example = user_profile::create(
///         "example",
///         UserProfileArgs::builder()
///             .domain_identifier("${exampleAwsDatazoneDomain.id}")
///             .user_identifier("${exampleAwsIamUser.arn}")
///             .user_type("IAM_USER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DataZone User Profile using the `user_identifier,domain_identifier,type`. For example:
///
/// ```sh
/// $ pulumi import aws:datazone/userProfile:UserProfile example arn:aws:iam::123456789012:user/example,dzd_54nakfrg9k6suo,IAM
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
        /// The domain identifier.
        #[builder(into)]
        pub domain_identifier: pulumi_gestalt_rust::Input<String>,
        /// The user profile status.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::Input<Option<String>>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::Input<
            Option<super::super::types::datazone::UserProfileTimeouts>,
        >,
        /// The user identifier.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_identifier: pulumi_gestalt_rust::Input<String>,
        /// The user type.
        #[builder(into, default)]
        pub user_type: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct UserProfileResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Details about the user profile.
        pub details: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datazone::UserProfileDetail>,
        >,
        /// The domain identifier.
        pub domain_identifier: pulumi_gestalt_rust::Output<String>,
        /// The user profile status.
        pub status: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::datazone::UserProfileTimeouts>,
        >,
        /// The user profile type.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The user identifier.
        ///
        /// The following arguments are optional:
        pub user_identifier: pulumi_gestalt_rust::Output<String>,
        /// The user type.
        pub user_type: pulumi_gestalt_rust::Output<String>,
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
        let domain_identifier_binding = args.domain_identifier.get_output(ctx);
        let status_binding = args.status.get_output(ctx);
        let timeouts_binding = args.timeouts.get_output(ctx);
        let user_identifier_binding = args.user_identifier.get_output(ctx);
        let user_type_binding = args.user_type.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:datazone/userProfile:UserProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainIdentifier".into(),
                    value: &domain_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userIdentifier".into(),
                    value: &user_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userType".into(),
                    value: &user_type_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        UserProfileResult {
            id: o.get_id(),
            urn: o.get_urn(),
            details: o.get_field("details"),
            domain_identifier: o.get_field("domainIdentifier"),
            status: o.get_field("status"),
            timeouts: o.get_field("timeouts"),
            type_: o.get_field("type"),
            user_identifier: o.get_field("userIdentifier"),
            user_type: o.get_field("userType"),
        }
    }
}
