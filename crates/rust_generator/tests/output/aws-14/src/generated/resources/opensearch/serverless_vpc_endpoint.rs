/// Resource for managing an AWS OpenSearchServerless VPC Endpoint.
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
///     let example = serverless_vpc_endpoint::create(
///         "example",
///         ServerlessVpcEndpointArgs::builder()
///             .name("myendpoint")
///             .subnet_ids(vec!["${exampleAwsSubnet.id}",])
///             .vpc_id("${exampleAwsVpc.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import OpenSearchServerless Vpc Endpointa using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/serverlessVpcEndpoint:ServerlessVpcEndpoint example vpce-8012925589
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod serverless_vpc_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerlessVpcEndpointArgs {
        /// Name of the interface endpoint.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// One or more security groups that define the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint. Up to 5 security groups can be provided.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
        /// One or more subnet IDs from which you'll access OpenSearch Serverless. Up to 6 subnets can be provided.
        #[builder(into)]
        pub subnet_ids: pulumi_gestalt_rust::Input<Vec<String>>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::Input<
            Option<super::super::types::opensearch::ServerlessVpcEndpointTimeouts>,
        >,
        /// ID of the VPC from which you'll access OpenSearch Serverless.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ServerlessVpcEndpointResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Name of the interface endpoint.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more security groups that define the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint. Up to 5 security groups can be provided.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// One or more subnet IDs from which you'll access OpenSearch Serverless. Up to 6 subnets can be provided.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::opensearch::ServerlessVpcEndpointTimeouts>,
        >,
        /// ID of the VPC from which you'll access OpenSearch Serverless.
        ///
        /// The following arguments are optional:
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServerlessVpcEndpointArgs,
    ) -> ServerlessVpcEndpointResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServerlessVpcEndpointArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ServerlessVpcEndpointResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServerlessVpcEndpointArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ServerlessVpcEndpointResult {
        let name_binding = args.name.get_output(ctx);
        let security_group_ids_binding = args.security_group_ids.get_output(ctx);
        let subnet_ids_binding = args.subnet_ids.get_output(ctx);
        let timeouts_binding = args.timeouts.get_output(ctx);
        let vpc_id_binding = args.vpc_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:opensearch/serverlessVpcEndpoint:ServerlessVpcEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ServerlessVpcEndpointResult {
            id: o.get_id(),
            urn: o.get_urn(),
            name: o.get_field("name"),
            security_group_ids: o.get_field("securityGroupIds"),
            subnet_ids: o.get_field("subnetIds"),
            timeouts: o.get_field("timeouts"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
