/// Provides an AppSync API Association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain_name_api_association::create(
///         "example",
///         DomainNameApiAssociationArgs::builder()
///             .api_id("${exampleAwsAppsyncGraphqlApi.id}")
///             .domain_name("${exampleAwsAppsyncDomainName.domainName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_appsync_domain_name_api_association` using the AppSync domain name. For example:
///
/// ```sh
/// $ pulumi import aws:appsync/domainNameApiAssociation:DomainNameApiAssociation example example.com
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod domain_name_api_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainNameApiAssociationArgs {
        /// API ID.
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::Input<String>,
        /// Appsync domain name.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct DomainNameApiAssociationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// API ID.
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// Appsync domain name.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainNameApiAssociationArgs,
    ) -> DomainNameApiAssociationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainNameApiAssociationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DomainNameApiAssociationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainNameApiAssociationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DomainNameApiAssociationResult {
        let api_id_binding = args.api_id.get_output(ctx);
        let domain_name_binding = args.domain_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appsync/domainNameApiAssociation:DomainNameApiAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DomainNameApiAssociationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            api_id: o.get_field("apiId"),
            domain_name: o.get_field("domainName"),
        }
    }
}
