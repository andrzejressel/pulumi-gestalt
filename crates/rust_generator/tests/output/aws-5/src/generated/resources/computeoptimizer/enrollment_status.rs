/// Manages AWS Compute Optimizer enrollment status.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = enrollment_status::create(
///         "example",
///         EnrollmentStatusArgs::builder().status("Active").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import enrollment status using the account ID. For example:
///
/// ```sh
/// $ pulumi import aws:computeoptimizer/enrollmentStatus:EnrollmentStatus example 123456789012
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
        /// Whether to enroll member accounts of the organization if the account is the management account of an organization. Default is `false`.
        #[builder(into, default)]
        pub include_member_accounts: pulumi_gestalt_rust::Input<Option<bool>>,
        /// The enrollment status of the account. Valid values: `Active`, `Inactive`.
        #[builder(into)]
        pub status: pulumi_gestalt_rust::Input<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::Input<
            Option<super::super::types::computeoptimizer::EnrollmentStatusTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnrollmentStatusResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Whether to enroll member accounts of the organization if the account is the management account of an organization. Default is `false`.
        pub include_member_accounts: pulumi_gestalt_rust::Output<bool>,
        /// The count of organization member accounts that are opted in to the service, if your account is an organization management account.
        pub number_of_member_accounts_opted_in: pulumi_gestalt_rust::Output<i32>,
        /// The enrollment status of the account. Valid values: `Active`, `Inactive`.
        pub status: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::computeoptimizer::EnrollmentStatusTimeouts>,
        >,
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
        let status_binding = args.status.get_output(ctx);
        let timeouts_binding = args.timeouts.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:computeoptimizer/enrollmentStatus:EnrollmentStatus".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeMemberAccounts".into(),
                    value: &include_member_accounts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        EnrollmentStatusResult {
            id: o.get_id(),
            urn: o.get_urn(),
            include_member_accounts: o.get_field("includeMemberAccounts"),
            number_of_member_accounts_opted_in: o
                .get_field("numberOfMemberAccountsOptedIn"),
            status: o.get_field("status"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
