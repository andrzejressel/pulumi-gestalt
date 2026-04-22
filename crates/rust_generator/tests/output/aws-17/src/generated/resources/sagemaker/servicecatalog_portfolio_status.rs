/// Manages status of Service Catalog in SageMaker. Service Catalog is used to create SageMaker projects.
///
/// ## Example Usage
///
/// Usage:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = servicecatalog_portfolio_status::create(
///         "example",
///         ServicecatalogPortfolioStatusArgs::builder().status("Enabled").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import models using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/servicecatalogPortfolioStatus:ServicecatalogPortfolioStatus example us-east-1
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod servicecatalog_portfolio_status {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServicecatalogPortfolioStatusArgs {
        /// Whether Service Catalog is enabled or disabled in SageMaker. Valid values are `Enabled` and `Disabled`.
        #[builder(into)]
        pub status: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ServicecatalogPortfolioStatusResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Whether Service Catalog is enabled or disabled in SageMaker. Valid values are `Enabled` and `Disabled`.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServicecatalogPortfolioStatusArgs,
    ) -> ServicecatalogPortfolioStatusResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServicecatalogPortfolioStatusArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ServicecatalogPortfolioStatusResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServicecatalogPortfolioStatusArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ServicecatalogPortfolioStatusResult {
        let status_binding = args.status.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/servicecatalogPortfolioStatus:ServicecatalogPortfolioStatus"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ServicecatalogPortfolioStatusResult {
            id: o.get_id(),
            urn: o.get_urn(),
            status: o.get_field("status"),
        }
    }
}
