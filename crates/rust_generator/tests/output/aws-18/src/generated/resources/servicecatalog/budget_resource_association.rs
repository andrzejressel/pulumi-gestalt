/// Manages a Service Catalog Budget Resource Association.
///
/// > **Tip:** A "resource" is either a Service Catalog portfolio or product.
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
///     let example = budget_resource_association::create(
///         "example",
///         BudgetResourceAssociationArgs::builder()
///             .budget_name("budget-pjtvyakdlyo3m")
///             .resource_id("prod-dnigbtea24ste")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicecatalog_budget_resource_association` using the budget name and resource ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/budgetResourceAssociation:BudgetResourceAssociation example budget-pjtvyakdlyo3m:prod-dnigbtea24ste
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod budget_resource_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BudgetResourceAssociationArgs {
        /// Budget name.
        #[builder(into)]
        pub budget_name: pulumi_gestalt_rust::Input<String>,
        /// Resource identifier.
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct BudgetResourceAssociationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Budget name.
        pub budget_name: pulumi_gestalt_rust::Output<String>,
        /// Resource identifier.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BudgetResourceAssociationArgs,
    ) -> BudgetResourceAssociationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BudgetResourceAssociationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> BudgetResourceAssociationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BudgetResourceAssociationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> BudgetResourceAssociationResult {
        let budget_name_binding = args.budget_name.get_output(ctx);
        let resource_id_binding = args.resource_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:servicecatalog/budgetResourceAssociation:BudgetResourceAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "budgetName".into(),
                    value: &budget_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        BudgetResourceAssociationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            budget_name: o.get_field("budgetName"),
            resource_id: o.get_field("resourceId"),
        }
    }
}
