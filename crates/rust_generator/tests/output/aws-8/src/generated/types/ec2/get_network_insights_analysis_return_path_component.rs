#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNetworkInsightsAnalysisReturnPathComponent {
    #[builder(into)]
    #[serde(rename = "aclRules")]
    pub r#acl_rules: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentAclRule>,
    #[builder(into)]
    #[serde(rename = "additionalDetails")]
    pub r#additional_details: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentAdditionalDetail>,
    #[builder(into)]
    #[serde(rename = "attachedTos")]
    pub r#attached_tos: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentAttachedTo>,
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentComponent>,
    #[builder(into)]
    #[serde(rename = "destinationVpcs")]
    pub r#destination_vpcs: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentDestinationVpc>,
    #[builder(into)]
    #[serde(rename = "inboundHeaders")]
    pub r#inbound_headers: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentInboundHeader>,
    #[builder(into)]
    #[serde(rename = "outboundHeaders")]
    pub r#outbound_headers: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentOutboundHeader>,
    #[builder(into)]
    #[serde(rename = "routeTableRoutes")]
    pub r#route_table_routes: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentRouteTableRoute>,
    #[builder(into)]
    #[serde(rename = "securityGroupRules")]
    pub r#security_group_rules: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentSecurityGroupRule>,
    #[builder(into)]
    #[serde(rename = "sequenceNumber")]
    pub r#sequence_number: i32,
    #[builder(into)]
    #[serde(rename = "sourceVpcs")]
    pub r#source_vpcs: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentSourceVpc>,
    #[builder(into)]
    #[serde(rename = "subnets")]
    pub r#subnets: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentSubnet>,
    #[builder(into)]
    #[serde(rename = "transitGatewayRouteTableRoutes")]
    pub r#transit_gateway_route_table_routes: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentTransitGatewayRouteTableRoute>,
    #[builder(into)]
    #[serde(rename = "transitGateways")]
    pub r#transit_gateways: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentTransitGateway>,
    #[builder(into)]
    #[serde(rename = "vpcs")]
    pub r#vpcs: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentVpc>,
}
