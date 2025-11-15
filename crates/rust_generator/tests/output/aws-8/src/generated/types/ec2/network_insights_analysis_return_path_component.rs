#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkInsightsAnalysisReturnPathComponent {
    #[builder(into)]
    #[serde(rename = "aclRules")]
    pub r#acl_rules: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentAclRule>>,
    #[builder(into)]
    #[serde(rename = "additionalDetails")]
    pub r#additional_details: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentAdditionalDetail>>,
    #[builder(into)]
    #[serde(rename = "attachedTos")]
    pub r#attached_tos: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentAttachedTo>>,
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentComponent>>,
    #[builder(into)]
    #[serde(rename = "destinationVpcs")]
    pub r#destination_vpcs: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentDestinationVpc>>,
    #[builder(into)]
    #[serde(rename = "inboundHeaders")]
    pub r#inbound_headers: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentInboundHeader>>,
    #[builder(into)]
    #[serde(rename = "outboundHeaders")]
    pub r#outbound_headers: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentOutboundHeader>>,
    #[builder(into)]
    #[serde(rename = "routeTableRoutes")]
    pub r#route_table_routes: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentRouteTableRoute>>,
    #[builder(into)]
    #[serde(rename = "securityGroupRules")]
    pub r#security_group_rules: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentSecurityGroupRule>>,
    #[builder(into)]
    #[serde(rename = "sequenceNumber")]
    pub r#sequence_number: Option<i32>,
    #[builder(into)]
    #[serde(rename = "sourceVpcs")]
    pub r#source_vpcs: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentSourceVpc>>,
    #[builder(into)]
    #[serde(rename = "subnets")]
    pub r#subnets: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentSubnet>>,
    #[builder(into)]
    #[serde(rename = "transitGatewayRouteTableRoutes")]
    pub r#transit_gateway_route_table_routes: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentTransitGatewayRouteTableRoute>>,
    #[builder(into)]
    #[serde(rename = "transitGateways")]
    pub r#transit_gateways: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentTransitGateway>>,
    #[builder(into)]
    #[serde(rename = "vpcs")]
    pub r#vpcs: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentVpc>>,
}
