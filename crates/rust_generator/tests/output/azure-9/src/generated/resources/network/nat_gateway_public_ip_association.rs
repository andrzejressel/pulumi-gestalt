/// Manages the association between a NAT Gateway and a Public IP.
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
///     let exampleNatGateway = nat_gateway::create(
///         "exampleNatGateway",
///         NatGatewayArgs::builder()
///             .location("${example.location}")
///             .name("example-NatGateway")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard")
///             .build_struct(),
///     );
///     let exampleNatGatewayPublicIpAssociation = nat_gateway_public_ip_association::create(
///         "exampleNatGatewayPublicIpAssociation",
///         NatGatewayPublicIpAssociationArgs::builder()
///             .nat_gateway_id("${exampleNatGateway.id}")
///             .public_ip_address_id("${examplePublicIp.id}")
///             .build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("${example.location}")
///             .name("example-PIP")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Associations between NAT Gateway and Public IP Addresses can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/natGatewayPublicIpAssociation:NatGatewayPublicIpAssociation example "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/natGateways/gateway1|/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/publicIPAddresses/myPublicIpAddress1"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod nat_gateway_public_ip_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NatGatewayPublicIpAssociationArgs {
        /// The ID of the NAT Gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub nat_gateway_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Public IP which this NAT Gateway which should be connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub public_ip_address_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NatGatewayPublicIpAssociationResult {
        /// The ID of the NAT Gateway. Changing this forces a new resource to be created.
        pub nat_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Public IP which this NAT Gateway which should be connected to. Changing this forces a new resource to be created.
        pub public_ip_address_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NatGatewayPublicIpAssociationArgs,
    ) -> NatGatewayPublicIpAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let nat_gateway_id_binding = args.nat_gateway_id.get_output(context);
        let public_ip_address_id_binding = args.public_ip_address_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/natGatewayPublicIpAssociation:NatGatewayPublicIpAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "natGatewayId".into(),
                    value: &nat_gateway_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicIpAddressId".into(),
                    value: &public_ip_address_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NatGatewayPublicIpAssociationResult {
            nat_gateway_id: o.get_field("natGatewayId"),
            public_ip_address_id: o.get_field("publicIpAddressId"),
        }
    }
}
