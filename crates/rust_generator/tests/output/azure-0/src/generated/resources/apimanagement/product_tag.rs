/// Manages an API Management Product tag
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleProduct = product::create(
///         "exampleProduct",
///         ProductArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .approval_required(true)
///             .display_name("Test Product")
///             .product_id("test-product")
///             .published(true)
///             .resource_group_name("${example.name}")
///             .subscription_required(true)
///             .build_struct(),
///     );
///     let exampleProductTag = product_tag::create(
///         "exampleProductTag",
///         ProductTagArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .api_management_product_id("${exampleProduct.productId}")
///             .name("${exampleTag.name}")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-apim")
///             .publisher_email("company@terraform.io")
///             .publisher_name("My Company")
///             .resource_group_name("${example.name}")
///             .sku_name("Developer_1")
///             .build_struct(),
///     );
///     let exampleTag = tag::create(
///         "exampleTag",
///         TagArgs::builder()
///             .api_management_id("${exampleService.id}")
///             .name("example-tag")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// API Management Products can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/productTag:ProductTag example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/products/myproduct/tags/mytag
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod product_tag {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProductTagArgs {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::Input<String>,
        /// The name of the API Management product. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_product_id: pulumi_gestalt_rust::Input<String>,
        /// The name which should be used for this API Management Tag. Changing this forces a new API Management Tag to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the Resource Group in which the API Management Service should be exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ProductTagResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the API Management product. Changing this forces a new resource to be created.
        pub api_management_product_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this API Management Tag. Changing this forces a new API Management Tag to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service should be exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProductTagArgs,
    ) -> ProductTagResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProductTagArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ProductTagResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProductTagArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ProductTagResult {
        let api_management_name_binding = args.api_management_name.get_output(ctx);
        let api_management_product_id_binding = args
            .api_management_product_id
            .get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/productTag:ProductTag".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementProductId".into(),
                    value: &api_management_product_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ProductTagResult {
            id: o.get_id(),
            urn: o.get_urn(),
            api_management_name: o.get_field("apiManagementName"),
            api_management_product_id: o.get_field("apiManagementProductId"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
