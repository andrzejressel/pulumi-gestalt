/// Resource for managing AWS Cost Optimization Hub Enrollment Status.
///
/// > **TIP:** The Cost Optimization Hub only has a `us-east-1` endpoint. However, you can access the service globally with the AWS Provider from other regions. Other tools, such as the [AWS CLI](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/cost-optimization-hub/index.html), may require you to specify the `us-east-1` region when using the service.
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
///     let example = enrollment_status::create(
///         "example",
///         EnrollmentStatusArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ### Usage with all the arguments
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = enrollment_status::create(
///         "example",
///         EnrollmentStatusArgs::builder().include_member_accounts(true).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cost Optimization Hub Enrollment Status using your AWS account ID. For example:
///
/// ```sh
/// $ pulumi import aws:costoptimizationhub/enrollmentStatus:EnrollmentStatus example 111222333444
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod enrollment_status {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnrollmentStatusArgs {
        /// Flag to enroll member accounts of the organization if the account is the management account. No drift detection is currently supported for this argument. Default value is `false`.
        #[builder(into, default)]
        pub include_member_accounts: pulumi_gestalt_rust::Input<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct EnrollmentStatusResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Flag to enroll member accounts of the organization if the account is the management account. No drift detection is currently supported for this argument. Default value is `false`.
        pub include_member_accounts: pulumi_gestalt_rust::Output<bool>,
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnrollmentStatusArgs,
    ) -> EnrollmentStatusResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnrollmentStatusArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> EnrollmentStatusResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnrollmentStatusArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> EnrollmentStatusResult {
        let include_member_accounts_binding = args
            .include_member_accounts
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:costoptimizationhub/enrollmentStatus:EnrollmentStatus".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeMemberAccounts".into(),
                    value: &include_member_accounts_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        EnrollmentStatusResult {
            id: o.get_id(),
            urn: o.get_urn(),
            include_member_accounts: o.get_field("includeMemberAccounts"),
            status: o.get_field("status"),
        }
    }
}
