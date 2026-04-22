/// Resource for managing an AWS SSO Admin Application Assignment.
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
///     let example = application_assignment::create(
///         "example",
///         ApplicationAssignmentArgs::builder()
///             .application_arn("${exampleAwsSsoadminApplication.applicationArn}")
///             .principal_id("${exampleAwsIdentitystoreUser.userId}")
///             .principal_type("USER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Group Type
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application_assignment::create(
///         "example",
///         ApplicationAssignmentArgs::builder()
///             .application_arn("${exampleAwsSsoadminApplication.applicationArn}")
///             .principal_id("${exampleAwsIdentitystoreGroup.groupId}")
///             .principal_type("GROUP")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Admin Application Assignment using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/applicationAssignment:ApplicationAssignment example arn:aws:sso::123456789012:application/id-12345678,abcd1234,USER
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod application_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationAssignmentArgs {
        /// ARN of the application.
        #[builder(into)]
        pub application_arn: pulumi_gestalt_rust::Input<String>,
        /// An identifier for an object in IAM Identity Center, such as a user or group.
        #[builder(into)]
        pub principal_id: pulumi_gestalt_rust::Input<String>,
        /// Entity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
        #[builder(into)]
        pub principal_type: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationAssignmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the application.
        pub application_arn: pulumi_gestalt_rust::Output<String>,
        /// An identifier for an object in IAM Identity Center, such as a user or group.
        pub principal_id: pulumi_gestalt_rust::Output<String>,
        /// Entity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
        pub principal_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationAssignmentArgs,
    ) -> ApplicationAssignmentResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationAssignmentArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ApplicationAssignmentResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationAssignmentArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ApplicationAssignmentResult {
        let application_arn_binding = args.application_arn.get_output(ctx);
        let principal_id_binding = args.principal_id.get_output(ctx);
        let principal_type_binding = args.principal_type.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssoadmin/applicationAssignment:ApplicationAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationArn".into(),
                    value: &application_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalId".into(),
                    value: &principal_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalType".into(),
                    value: &principal_type_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ApplicationAssignmentResult {
            id: o.get_id(),
            urn: o.get_urn(),
            application_arn: o.get_field("applicationArn"),
            principal_id: o.get_field("principalId"),
            principal_type: o.get_field("principalType"),
        }
    }
}
