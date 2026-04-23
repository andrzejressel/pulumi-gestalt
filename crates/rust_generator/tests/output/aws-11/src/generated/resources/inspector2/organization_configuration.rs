/// Resource for managing an Amazon Inspector Organization Configuration.
///
/// > **NOTE:** In order for this resource to work, the account you use must be an Inspector Delegated Admin Account.
///
/// > **NOTE:** When this resource is deleted, EC2, ECR, Lambda, and Lambda code scans will no longer be automatically enabled for new members of your Amazon Inspector organization.
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
///     let example = organization_configuration::create(
///         "example",
///         OrganizationConfigurationArgs::builder()
///             .auto_enable(
///                 OrganizationConfigurationAutoEnable::builder()
///                     .ec2(true)
///                     .ecr(false)
///                     .lambda(true)
///                     .lambdaCode(true)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod organization_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationConfigurationArgs {
        /// Configuration block for auto enabling. See below.
        #[builder(into)]
        pub auto_enable: pulumi_gestalt_rust::Input<
            super::super::types::inspector2::OrganizationConfigurationAutoEnable,
        >,
    }
    #[allow(dead_code)]
    pub struct OrganizationConfigurationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for auto enabling. See below.
        pub auto_enable: pulumi_gestalt_rust::Output<
            super::super::types::inspector2::OrganizationConfigurationAutoEnable,
        >,
        /// Whether your configuration reached the max account limit.
        pub max_account_limit_reached: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationConfigurationArgs,
    ) -> OrganizationConfigurationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationConfigurationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> OrganizationConfigurationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationConfigurationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> OrganizationConfigurationResult {
        let auto_enable_binding = args.auto_enable.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:inspector2/organizationConfiguration:OrganizationConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoEnable".into(),
                    value: &auto_enable_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        OrganizationConfigurationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            auto_enable: o.get_field("autoEnable"),
            max_account_limit_reached: o.get_field("maxAccountLimitReached"),
        }
    }
}
