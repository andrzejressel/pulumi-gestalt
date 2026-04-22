/// Provides a Route53 CIDR location resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cidr_collection::create(
///         "example",
///         CidrCollectionArgs::builder().name("collection-1").build_struct(),
///     );
///     let exampleCidrLocation = cidr_location::create(
///         "exampleCidrLocation",
///         CidrLocationArgs::builder()
///             .cidr_blocks(vec!["200.5.3.0/24", "200.6.3.0/24",])
///             .cidr_collection_id("${example.id}")
///             .name("office")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CIDR locations using their the CIDR collection ID and location name. For example:
///
/// ```sh
/// $ pulumi import aws:route53/cidrLocation:CidrLocation example 9ac32814-3e67-0932-6048-8d779cc6f511,office
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod cidr_location {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CidrLocationArgs {
        /// CIDR blocks for the location.
        #[builder(into)]
        pub cidr_blocks: pulumi_gestalt_rust::Input<Vec<String>>,
        /// The ID of the CIDR collection to update.
        #[builder(into)]
        pub cidr_collection_id: pulumi_gestalt_rust::Input<String>,
        /// Name for the CIDR location.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CidrLocationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// CIDR blocks for the location.
        pub cidr_blocks: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the CIDR collection to update.
        pub cidr_collection_id: pulumi_gestalt_rust::Output<String>,
        /// Name for the CIDR location.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CidrLocationArgs,
    ) -> CidrLocationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CidrLocationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> CidrLocationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CidrLocationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> CidrLocationResult {
        let cidr_blocks_binding = args.cidr_blocks.get_output(ctx);
        let cidr_collection_id_binding = args.cidr_collection_id.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/cidrLocation:CidrLocation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidrBlocks".into(),
                    value: &cidr_blocks_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidrCollectionId".into(),
                    value: &cidr_collection_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        CidrLocationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            cidr_blocks: o.get_field("cidrBlocks"),
            cidr_collection_id: o.get_field("cidrCollectionId"),
            name: o.get_field("name"),
        }
    }
}
