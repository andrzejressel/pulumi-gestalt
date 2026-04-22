/// Manages an [AWS Opensearch VPC Endpoint](https://docs.aws.amazon.com/opensearch-service/latest/APIReference/API_CreateVpcEndpoint.html). Creates an Amazon OpenSearch Service-managed VPC endpoint.
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
///     let foo = vpc_endpoint::create(
///         "foo",
///         VpcEndpointArgs::builder()
///             .domain_arn("${domain1.arn}")
///             .vpc_options(
///                 VpcEndpointVpcOptions::builder()
///                     .securityGroupIds(vec!["${test.id}", "${test2.id}",])
///                     .subnetIds(vec!["${testAwsSubnet.id}", "${test2AwsSubnet.id}",])
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import OpenSearch VPC endpoint connections using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/vpcEndpoint:VpcEndpoint example endpoint-id
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod vpc_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointArgs {
        /// Specifies the Amazon Resource Name (ARN) of the domain to create the endpoint for
        #[builder(into)]
        pub domain_arn: pulumi_gestalt_rust::Input<String>,
        /// Options to specify the subnets and security groups for the endpoint.
        #[builder(into)]
        pub vpc_options: pulumi_gestalt_rust::Input<
            super::super::types::opensearch::VpcEndpointVpcOptions,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Amazon Resource Name (ARN) of the domain to create the endpoint for
        pub domain_arn: pulumi_gestalt_rust::Output<String>,
        /// The connection endpoint ID for connecting to the domain.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Options to specify the subnets and security groups for the endpoint.
        pub vpc_options: pulumi_gestalt_rust::Output<
            super::super::types::opensearch::VpcEndpointVpcOptions,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointArgs,
    ) -> VpcEndpointResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> VpcEndpointResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> VpcEndpointResult {
        let domain_arn_binding = args.domain_arn.get_output(ctx);
        let vpc_options_binding = args.vpc_options.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:opensearch/vpcEndpoint:VpcEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainArn".into(),
                    value: &domain_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcOptions".into(),
                    value: &vpc_options_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        VpcEndpointResult {
            id: o.get_id(),
            urn: o.get_urn(),
            domain_arn: o.get_field("domainArn"),
            endpoint: o.get_field("endpoint"),
            vpc_options: o.get_field("vpcOptions"),
        }
    }
}
