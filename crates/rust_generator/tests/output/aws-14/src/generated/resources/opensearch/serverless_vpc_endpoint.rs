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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod serverless_vpc_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerlessVpcEndpointArgs {
        /// Name of the interface endpoint.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more security groups that define the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint. Up to 5 security groups can be provided.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// One or more subnet IDs from which you'll access OpenSearch Serverless. Up to 6 subnets can be provided.
        #[builder(into)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::opensearch::ServerlessVpcEndpointTimeouts>,
        >,
        /// ID of the VPC from which you'll access OpenSearch Serverless.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServerlessVpcEndpointResult {
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServerlessVpcEndpointArgs,
    ) -> ServerlessVpcEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let security_group_ids_binding = args.security_group_ids.get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
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
        };
        let o = context.register_resource(request);
        ServerlessVpcEndpointResult {
            name: o.get_field("name"),
            security_group_ids: o.get_field("securityGroupIds"),
            subnet_ids: o.get_field("subnetIds"),
            timeouts: o.get_field("timeouts"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
