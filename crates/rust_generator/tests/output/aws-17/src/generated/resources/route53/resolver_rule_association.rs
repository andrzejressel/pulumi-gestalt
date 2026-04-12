/// Provides a Route53 Resolver rule association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resolver_rule_association::create(
///         "example",
///         ResolverRuleAssociationArgs::builder()
///             .resolver_rule_id("${sys.id}")
///             .vpc_id("${foo.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Resolver rule associations using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:route53/resolverRuleAssociation:ResolverRuleAssociation example rslvr-rrassoc-97242eaf88example
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod resolver_rule_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverRuleAssociationArgs {
        /// A name for the association that you're creating between a resolver rule and a VPC.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the resolver rule that you want to associate with the VPC.
        #[builder(into)]
        pub resolver_rule_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the VPC that you want to associate the resolver rule with.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResolverRuleAssociationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// A name for the association that you're creating between a resolver rule and a VPC.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the resolver rule that you want to associate with the VPC.
        pub resolver_rule_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VPC that you want to associate the resolver rule with.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResolverRuleAssociationArgs,
    ) -> ResolverRuleAssociationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResolverRuleAssociationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ResolverRuleAssociationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResolverRuleAssociationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ResolverRuleAssociationResult {
        let name_binding = args.name.get_output(ctx);
        let resolver_rule_id_binding = args.resolver_rule_id.get_output(ctx);
        let vpc_id_binding = args.vpc_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/resolverRuleAssociation:ResolverRuleAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resolverRuleId".into(),
                    value: &resolver_rule_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ResolverRuleAssociationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            name: o.get_field("name"),
            resolver_rule_id: o.get_field("resolverRuleId"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
