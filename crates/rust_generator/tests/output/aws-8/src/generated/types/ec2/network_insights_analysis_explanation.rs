#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkInsightsAnalysisExplanation {
    #[builder(into)]
    #[serde(rename = "aclRules")]
    pub r#acl_rules: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationAclRule>>,
    #[builder(into)]
    #[serde(rename = "acls")]
    pub r#acls: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationAcl>>,
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    #[builder(into)]
    #[serde(rename = "addresses")]
    pub r#addresses: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "attachedTos")]
    pub r#attached_tos: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationAttachedTo>>,
    #[builder(into)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "cidrs")]
    pub r#cidrs: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "classicLoadBalancerListeners")]
    pub r#classic_load_balancer_listeners: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationClassicLoadBalancerListener>>,
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationComponent>>,
    #[builder(into)]
    #[serde(rename = "customerGateways")]
    pub r#customer_gateways: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationCustomerGateway>>,
    #[builder(into)]
    #[serde(rename = "destinationVpcs")]
    pub r#destination_vpcs: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationDestinationVpc>>,
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationDestination>>,
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: Option<String>,
    #[builder(into)]
    #[serde(rename = "elasticLoadBalancerListeners")]
    pub r#elastic_load_balancer_listeners: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationElasticLoadBalancerListener>>,
    #[builder(into)]
    #[serde(rename = "explanationCode")]
    pub r#explanation_code: Option<String>,
    #[builder(into)]
    #[serde(rename = "ingressRouteTables")]
    pub r#ingress_route_tables: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationIngressRouteTable>>,
    #[builder(into)]
    #[serde(rename = "internetGateways")]
    pub r#internet_gateways: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationInternetGateway>>,
    #[builder(into)]
    #[serde(rename = "loadBalancerArn")]
    pub r#load_balancer_arn: Option<String>,
    #[builder(into)]
    #[serde(rename = "loadBalancerListenerPort")]
    pub r#load_balancer_listener_port: Option<i32>,
    #[builder(into)]
    #[serde(rename = "loadBalancerTargetGroup")]
    pub r#load_balancer_target_group: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationLoadBalancerTargetGroup>>,
    #[builder(into)]
    #[serde(rename = "loadBalancerTargetGroups")]
    pub r#load_balancer_target_groups: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationLoadBalancerTargetGroup>>,
    #[builder(into)]
    #[serde(rename = "loadBalancerTargetPort")]
    pub r#load_balancer_target_port: Option<i32>,
    #[builder(into)]
    #[serde(rename = "missingComponent")]
    pub r#missing_component: Option<String>,
    #[builder(into)]
    #[serde(rename = "natGateways")]
    pub r#nat_gateways: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationNatGateway>>,
    #[builder(into)]
    #[serde(rename = "networkInterfaces")]
    pub r#network_interfaces: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationNetworkInterface>>,
    #[builder(into)]
    #[serde(rename = "packetField")]
    pub r#packet_field: Option<String>,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    #[builder(into)]
    #[serde(rename = "portRanges")]
    pub r#port_ranges: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationPortRange>>,
    #[builder(into)]
    #[serde(rename = "prefixLists")]
    pub r#prefix_lists: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationPrefixList>>,
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "routeTableRoutes")]
    pub r#route_table_routes: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationRouteTableRoute>>,
    #[builder(into)]
    #[serde(rename = "routeTables")]
    pub r#route_tables: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationRouteTable>>,
    #[builder(into)]
    #[serde(rename = "securityGroup")]
    pub r#security_group: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationSecurityGroup>>,
    #[builder(into)]
    #[serde(rename = "securityGroupRules")]
    pub r#security_group_rules: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationSecurityGroupRule>>,
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationSecurityGroup>>,
    #[builder(into)]
    #[serde(rename = "sourceVpcs")]
    pub r#source_vpcs: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationSourceVpc>>,
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    #[builder(into)]
    #[serde(rename = "subnetRouteTables")]
    pub r#subnet_route_tables: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationSubnetRouteTable>>,
    #[builder(into)]
    #[serde(rename = "subnets")]
    pub r#subnets: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationSubnet>>,
    #[builder(into)]
    #[serde(rename = "transitGatewayAttachments")]
    pub r#transit_gateway_attachments: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationTransitGatewayAttachment>>,
    #[builder(into)]
    #[serde(rename = "transitGatewayRouteTableRoutes")]
    pub r#transit_gateway_route_table_routes: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationTransitGatewayRouteTableRoute>>,
    #[builder(into)]
    #[serde(rename = "transitGatewayRouteTables")]
    pub r#transit_gateway_route_tables: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationTransitGatewayRouteTable>>,
    #[builder(into)]
    #[serde(rename = "transitGateways")]
    pub r#transit_gateways: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationTransitGateway>>,
    #[builder(into)]
    #[serde(rename = "vpcEndpoints")]
    pub r#vpc_endpoints: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationVpcEndpoint>>,
    #[builder(into)]
    #[serde(rename = "vpcPeeringConnections")]
    pub r#vpc_peering_connections: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationVpcPeeringConnection>>,
    #[builder(into)]
    #[serde(rename = "vpcs")]
    pub r#vpcs: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationVpc>>,
    #[builder(into)]
    #[serde(rename = "vpnConnections")]
    pub r#vpn_connections: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationVpnConnection>>,
    #[builder(into)]
    #[serde(rename = "vpnGateways")]
    pub r#vpn_gateways: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationVpnGateway>>,
}
