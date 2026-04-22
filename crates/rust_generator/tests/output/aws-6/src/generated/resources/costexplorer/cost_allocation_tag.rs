/// Provides a CE Cost Allocation Tag.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cost_allocation_tag::create(
///         "example",
///         CostAllocationTagArgs::builder()
///             .status("Active")
///             .tag_key("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ce_cost_allocation_tag` using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:costexplorer/costAllocationTag:CostAllocationTag example key
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod cost_allocation_tag {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CostAllocationTagArgs {
        /// The status of a cost allocation tag. Valid values are `Active` and `Inactive`.
        #[builder(into)]
        pub status: pulumi_gestalt_rust::Input<String>,
        /// The key for the cost allocation tag.
        #[builder(into)]
        pub tag_key: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct CostAllocationTagResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The status of a cost allocation tag. Valid values are `Active` and `Inactive`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The key for the cost allocation tag.
        pub tag_key: pulumi_gestalt_rust::Output<String>,
        /// The type of cost allocation tag.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CostAllocationTagArgs,
    ) -> CostAllocationTagResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CostAllocationTagArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> CostAllocationTagResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CostAllocationTagArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> CostAllocationTagResult {
        let status_binding = args.status.get_output(ctx);
        let tag_key_binding = args.tag_key.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:costexplorer/costAllocationTag:CostAllocationTag".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tagKey".into(),
                    value: &tag_key_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        CostAllocationTagResult {
            id: o.get_id(),
            urn: o.get_urn(),
            status: o.get_field("status"),
            tag_key: o.get_field("tagKey"),
            type_: o.get_field("type"),
        }
    }
}
