/// Creates a domain resource for the specified domain (e.g., example.com).
/// You cannot register a new domain name using Lightsail. You must register
/// a domain name using Amazon Route 53 or another domain name registrar.
/// If you have already registered your domain, you can enter its name in
/// this parameter to manage the DNS records for that domain.
///
/// > **Note:** Lightsail is currently only supported in a limited number of AWS Regions, please see ["Regions and Availability Zones in Amazon Lightsail"](https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail) for more details
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let domainTest = domain::create(
///         "domainTest",
///         DomainArgs::builder().domain_name("mydomain.com").build_struct(),
///     );
/// }
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// The name of the Lightsail domain to manage
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the Lightsail domain
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the Lightsail domain to manage
        pub domain_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainArgs,
    ) -> DomainResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DomainResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DomainResult {
        let domain_name_binding = args.domain_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/domain:Domain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DomainResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            domain_name: o.get_field("domainName"),
        }
    }
}
