/// Provides a resource to accept a pending VPC Endpoint Connection accept request to VPC Endpoint Service.
///
/// ## Example Usage
///
/// ### Accept cross-account request
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpc_endpoint_service::create(
///         "example",
///         VpcEndpointServiceArgs::builder()
///             .acceptance_required(false)
///             .network_load_balancer_arns(vec!["${exampleAwsLb.arn}",])
///             .build_struct(),
///     );
///     let exampleVpcEndpoint = vpc_endpoint::create(
///         "exampleVpcEndpoint",
///         VpcEndpointArgs::builder()
///             .private_dns_enabled(false)
///             .security_group_ids(vec!["${test.id}",])
///             .service_name("${testAwsVpcEndpointService.serviceName}")
///             .vpc_endpoint_type("Interface")
///             .vpc_id("${testAlternate.id}")
///             .build_struct(),
///     );
///     let exampleVpcEndpointConnectionAccepter = vpc_endpoint_connection_accepter::create(
///         "exampleVpcEndpointConnectionAccepter",
///         VpcEndpointConnectionAccepterArgs::builder()
///             .vpc_endpoint_id("${exampleVpcEndpoint.id}")
///             .vpc_endpoint_service_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Endpoint Services using ID of the connection, which is the `VPC Endpoint Service ID` and `VPC Endpoint ID` separated by underscore (`_`).. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcEndpointConnectionAccepter:VpcEndpointConnectionAccepter foo vpce-svc-0f97a19d3fa8220bc_vpce-010601a6db371e263
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_endpoint_connection_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointConnectionAccepterArgs {
        /// AWS VPC Endpoint ID.
        #[builder(into)]
        pub vpc_endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS VPC Endpoint Service ID.
        #[builder(into)]
        pub vpc_endpoint_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointConnectionAccepterResult {
        /// AWS VPC Endpoint ID.
        pub vpc_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// AWS VPC Endpoint Service ID.
        pub vpc_endpoint_service_id: pulumi_gestalt_rust::Output<String>,
        /// State of the VPC Endpoint.
        pub vpc_endpoint_state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointConnectionAccepterArgs,
    ) -> VpcEndpointConnectionAccepterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let vpc_endpoint_id_binding = args.vpc_endpoint_id.get_output(context);
        let vpc_endpoint_service_id_binding = args
            .vpc_endpoint_service_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointConnectionAccepter:VpcEndpointConnectionAccepter"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcEndpointId".into(),
                    value: &vpc_endpoint_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcEndpointServiceId".into(),
                    value: &vpc_endpoint_service_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcEndpointConnectionAccepterResult {
            vpc_endpoint_id: o.get_field("vpcEndpointId"),
            vpc_endpoint_service_id: o.get_field("vpcEndpointServiceId"),
            vpc_endpoint_state: o.get_field("vpcEndpointState"),
        }
    }
}
