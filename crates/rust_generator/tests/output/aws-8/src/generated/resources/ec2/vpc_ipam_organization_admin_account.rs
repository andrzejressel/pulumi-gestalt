/// Enables the IPAM Service and promotes a delegated administrator.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:VpcIpamOrganizationAdminAccount
///     properties:
///       delegatedAdminAccountId: ${delegated.accountId}
/// variables:
///   delegated:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IPAMs using the delegate account `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcIpamOrganizationAdminAccount:VpcIpamOrganizationAdminAccount example 12345678901
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod vpc_ipam_organization_admin_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamOrganizationAdminAccountArgs {
        #[builder(into)]
        pub delegated_admin_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcIpamOrganizationAdminAccountResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The Organizations ARN for the delegate account.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub delegated_admin_account_id: pulumi_gestalt_rust::Output<String>,
        /// The Organizations email for the delegate account.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The Organizations name for the delegate account.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The AWS service principal.
        pub service_principal: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcIpamOrganizationAdminAccountArgs,
    ) -> VpcIpamOrganizationAdminAccountResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcIpamOrganizationAdminAccountArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> VpcIpamOrganizationAdminAccountResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcIpamOrganizationAdminAccountArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> VpcIpamOrganizationAdminAccountResult {
        let delegated_admin_account_id_binding = args
            .delegated_admin_account_id
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamOrganizationAdminAccount:VpcIpamOrganizationAdminAccount"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "delegatedAdminAccountId".into(),
                    value: &delegated_admin_account_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        VpcIpamOrganizationAdminAccountResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            delegated_admin_account_id: o.get_field("delegatedAdminAccountId"),
            email: o.get_field("email"),
            name: o.get_field("name"),
            service_principal: o.get_field("servicePrincipal"),
        }
    }
}
