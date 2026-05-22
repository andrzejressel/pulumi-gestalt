#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkInsightsAnalysisExplanation {
    #[builder(into)]
    #[serde(rename = "aclRules")]
    pub r#acl_rules: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationAclRule>,
    #[builder(into)]
    #[serde(rename = "acls")]
    pub r#acls: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationAcl>,
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: String,
    #[builder(into)]
    #[serde(rename = "addresses")]
    pub r#addresses: Vec<String>,
    #[builder(into)]
    #[serde(rename = "attachedTos")]
    pub r#attached_tos: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationAttachedTo>,
    #[builder(into)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Vec<String>,
    #[builder(into)]
    #[serde(rename = "cidrs")]
    pub r#cidrs: Vec<String>,
    #[builder(into)]
    #[serde(rename = "classicLoadBalancerListeners")]
    pub r#classic_load_balancer_listeners: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationClassicLoadBalancerListener>,
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationComponent>,
    #[builder(into)]
    #[serde(rename = "customerGateways")]
    pub r#customer_gateways: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationCustomerGateway>,
    #[builder(into)]
    #[serde(rename = "destinationVpcs")]
    pub r#destination_vpcs: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationDestinationVpc>,
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationDestination>,
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: String,
    #[builder(into)]
    #[serde(rename = "elasticLoadBalancerListeners")]
    pub r#elastic_load_balancer_listeners: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationElasticLoadBalancerListener>,
    #[builder(into)]
    #[serde(rename = "explanationCode")]
    pub r#explanation_code: String,
    #[builder(into)]
    #[serde(rename = "ingressRouteTables")]
    pub r#ingress_route_tables: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationIngressRouteTable>,
    #[builder(into)]
    #[serde(rename = "internetGateways")]
    pub r#internet_gateways: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationInternetGateway>,
    #[builder(into)]
    #[serde(rename = "loadBalancerArn")]
    pub r#load_balancer_arn: String,
    #[builder(into)]
    #[serde(rename = "loadBalancerListenerPort")]
    pub r#load_balancer_listener_port: i32,
    #[builder(into)]
    #[serde(rename = "loadBalancerTargetGroup")]
    pub r#load_balancer_target_group: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationLoadBalancerTargetGroup>,
    #[builder(into)]
    #[serde(rename = "loadBalancerTargetGroups")]
    pub r#load_balancer_target_groups: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationLoadBalancerTargetGroup>,
    #[builder(into)]
    #[serde(rename = "loadBalancerTargetPort")]
    pub r#load_balancer_target_port: i32,
    #[builder(into)]
    #[serde(rename = "missingComponent")]
    pub r#missing_component: String,
    #[builder(into)]
    #[serde(rename = "natGateways")]
    pub r#nat_gateways: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationNatGateway>,
    #[builder(into)]
    #[serde(rename = "networkInterfaces")]
    pub r#network_interfaces: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationNetworkInterface>,
    #[builder(into)]
    #[serde(rename = "packetField")]
    pub r#packet_field: String,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    #[builder(into)]
    #[serde(rename = "portRanges")]
    pub r#port_ranges: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationPortRange>,
    #[builder(into)]
    #[serde(rename = "prefixLists")]
    pub r#prefix_lists: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationPrefixList>,
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Vec<String>,
    #[builder(into)]
    #[serde(rename = "routeTableRoutes")]
    pub r#route_table_routes: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationRouteTableRoute>,
    #[builder(into)]
    #[serde(rename = "routeTables")]
    pub r#route_tables: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationRouteTable>,
    #[builder(into)]
    #[serde(rename = "securityGroup")]
    pub r#security_group: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationSecurityGroup>,
    #[builder(into)]
    #[serde(rename = "securityGroupRules")]
    pub r#security_group_rules: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationSecurityGroupRule>,
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationSecurityGroup>,
    #[builder(into)]
    #[serde(rename = "sourceVpcs")]
    pub r#source_vpcs: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationSourceVpc>,
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
    #[builder(into)]
    #[serde(rename = "subnetRouteTables")]
    pub r#subnet_route_tables: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationSubnetRouteTable>,
    #[builder(into)]
    #[serde(rename = "subnets")]
    pub r#subnets: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationSubnet>,
    #[builder(into)]
    #[serde(rename = "transitGatewayAttachments")]
    pub r#transit_gateway_attachments: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationTransitGatewayAttachment>,
    #[builder(into)]
    #[serde(rename = "transitGatewayRouteTableRoutes")]
    pub r#transit_gateway_route_table_routes: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationTransitGatewayRouteTableRoute>,
    #[builder(into)]
    #[serde(rename = "transitGatewayRouteTables")]
    pub r#transit_gateway_route_tables: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationTransitGatewayRouteTable>,
    #[builder(into)]
    #[serde(rename = "transitGateways")]
    pub r#transit_gateways: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationTransitGateway>,
    #[builder(into)]
    #[serde(rename = "vpcEndpoints")]
    pub r#vpc_endpoints: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationVpcEndpoint>,
    #[builder(into)]
    #[serde(rename = "vpcPeeringConnections")]
    pub r#vpc_peering_connections: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationVpcPeeringConnection>,
    #[builder(into)]
    #[serde(rename = "vpcs")]
    pub r#vpcs: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationVpc>,
    #[builder(into)]
    #[serde(rename = "vpnConnections")]
    pub r#vpn_connections: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationVpnConnection>,
    #[builder(into)]
    #[serde(rename = "vpnGateways")]
    pub r#vpn_gateways: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationVpnGateway>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNetworkInsightsAnalysisExplanation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "aclRules",
                    &self.r#acl_rules,
                ),
                to_pulumi_object_field(
                    "acls",
                    &self.r#acls,
                ),
                to_pulumi_object_field(
                    "address",
                    &self.r#address,
                ),
                to_pulumi_object_field(
                    "addresses",
                    &self.r#addresses,
                ),
                to_pulumi_object_field(
                    "attachedTos",
                    &self.r#attached_tos,
                ),
                to_pulumi_object_field(
                    "availabilityZones",
                    &self.r#availability_zones,
                ),
                to_pulumi_object_field(
                    "cidrs",
                    &self.r#cidrs,
                ),
                to_pulumi_object_field(
                    "classicLoadBalancerListeners",
                    &self.r#classic_load_balancer_listeners,
                ),
                to_pulumi_object_field(
                    "components",
                    &self.r#components,
                ),
                to_pulumi_object_field(
                    "customerGateways",
                    &self.r#customer_gateways,
                ),
                to_pulumi_object_field(
                    "destinationVpcs",
                    &self.r#destination_vpcs,
                ),
                to_pulumi_object_field(
                    "destinations",
                    &self.r#destinations,
                ),
                to_pulumi_object_field(
                    "direction",
                    &self.r#direction,
                ),
                to_pulumi_object_field(
                    "elasticLoadBalancerListeners",
                    &self.r#elastic_load_balancer_listeners,
                ),
                to_pulumi_object_field(
                    "explanationCode",
                    &self.r#explanation_code,
                ),
                to_pulumi_object_field(
                    "ingressRouteTables",
                    &self.r#ingress_route_tables,
                ),
                to_pulumi_object_field(
                    "internetGateways",
                    &self.r#internet_gateways,
                ),
                to_pulumi_object_field(
                    "loadBalancerArn",
                    &self.r#load_balancer_arn,
                ),
                to_pulumi_object_field(
                    "loadBalancerListenerPort",
                    &self.r#load_balancer_listener_port,
                ),
                to_pulumi_object_field(
                    "loadBalancerTargetGroup",
                    &self.r#load_balancer_target_group,
                ),
                to_pulumi_object_field(
                    "loadBalancerTargetGroups",
                    &self.r#load_balancer_target_groups,
                ),
                to_pulumi_object_field(
                    "loadBalancerTargetPort",
                    &self.r#load_balancer_target_port,
                ),
                to_pulumi_object_field(
                    "missingComponent",
                    &self.r#missing_component,
                ),
                to_pulumi_object_field(
                    "natGateways",
                    &self.r#nat_gateways,
                ),
                to_pulumi_object_field(
                    "networkInterfaces",
                    &self.r#network_interfaces,
                ),
                to_pulumi_object_field(
                    "packetField",
                    &self.r#packet_field,
                ),
                to_pulumi_object_field(
                    "port",
                    &self.r#port,
                ),
                to_pulumi_object_field(
                    "portRanges",
                    &self.r#port_ranges,
                ),
                to_pulumi_object_field(
                    "prefixLists",
                    &self.r#prefix_lists,
                ),
                to_pulumi_object_field(
                    "protocols",
                    &self.r#protocols,
                ),
                to_pulumi_object_field(
                    "routeTableRoutes",
                    &self.r#route_table_routes,
                ),
                to_pulumi_object_field(
                    "routeTables",
                    &self.r#route_tables,
                ),
                to_pulumi_object_field(
                    "securityGroup",
                    &self.r#security_group,
                ),
                to_pulumi_object_field(
                    "securityGroupRules",
                    &self.r#security_group_rules,
                ),
                to_pulumi_object_field(
                    "securityGroups",
                    &self.r#security_groups,
                ),
                to_pulumi_object_field(
                    "sourceVpcs",
                    &self.r#source_vpcs,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
                to_pulumi_object_field(
                    "subnetRouteTables",
                    &self.r#subnet_route_tables,
                ),
                to_pulumi_object_field(
                    "subnets",
                    &self.r#subnets,
                ),
                to_pulumi_object_field(
                    "transitGatewayAttachments",
                    &self.r#transit_gateway_attachments,
                ),
                to_pulumi_object_field(
                    "transitGatewayRouteTableRoutes",
                    &self.r#transit_gateway_route_table_routes,
                ),
                to_pulumi_object_field(
                    "transitGatewayRouteTables",
                    &self.r#transit_gateway_route_tables,
                ),
                to_pulumi_object_field(
                    "transitGateways",
                    &self.r#transit_gateways,
                ),
                to_pulumi_object_field(
                    "vpcEndpoints",
                    &self.r#vpc_endpoints,
                ),
                to_pulumi_object_field(
                    "vpcPeeringConnections",
                    &self.r#vpc_peering_connections,
                ),
                to_pulumi_object_field(
                    "vpcs",
                    &self.r#vpcs,
                ),
                to_pulumi_object_field(
                    "vpnConnections",
                    &self.r#vpn_connections,
                ),
                to_pulumi_object_field(
                    "vpnGateways",
                    &self.r#vpn_gateways,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetNetworkInsightsAnalysisExplanation {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#acl_rules: {
                        let field_value = match fields_map.get("aclRules") {
                            Some(value) => value,
                            None => bail!("Missing field 'aclRules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#acls: {
                        let field_value = match fields_map.get("acls") {
                            Some(value) => value,
                            None => bail!("Missing field 'acls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#address: {
                        let field_value = match fields_map.get("address") {
                            Some(value) => value,
                            None => bail!("Missing field 'address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#addresses: {
                        let field_value = match fields_map.get("addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#attached_tos: {
                        let field_value = match fields_map.get("attachedTos") {
                            Some(value) => value,
                            None => bail!("Missing field 'attachedTos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#availability_zones: {
                        let field_value = match fields_map.get("availabilityZones") {
                            Some(value) => value,
                            None => bail!("Missing field 'availabilityZones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cidrs: {
                        let field_value = match fields_map.get("cidrs") {
                            Some(value) => value,
                            None => bail!("Missing field 'cidrs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#classic_load_balancer_listeners: {
                        let field_value = match fields_map.get("classicLoadBalancerListeners") {
                            Some(value) => value,
                            None => bail!("Missing field 'classicLoadBalancerListeners' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#components: {
                        let field_value = match fields_map.get("components") {
                            Some(value) => value,
                            None => bail!("Missing field 'components' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#customer_gateways: {
                        let field_value = match fields_map.get("customerGateways") {
                            Some(value) => value,
                            None => bail!("Missing field 'customerGateways' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_vpcs: {
                        let field_value = match fields_map.get("destinationVpcs") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationVpcs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destinations: {
                        let field_value = match fields_map.get("destinations") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#direction: {
                        let field_value = match fields_map.get("direction") {
                            Some(value) => value,
                            None => bail!("Missing field 'direction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#elastic_load_balancer_listeners: {
                        let field_value = match fields_map.get("elasticLoadBalancerListeners") {
                            Some(value) => value,
                            None => bail!("Missing field 'elasticLoadBalancerListeners' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#explanation_code: {
                        let field_value = match fields_map.get("explanationCode") {
                            Some(value) => value,
                            None => bail!("Missing field 'explanationCode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ingress_route_tables: {
                        let field_value = match fields_map.get("ingressRouteTables") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingressRouteTables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#internet_gateways: {
                        let field_value = match fields_map.get("internetGateways") {
                            Some(value) => value,
                            None => bail!("Missing field 'internetGateways' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_arn: {
                        let field_value = match fields_map.get("loadBalancerArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'loadBalancerArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_listener_port: {
                        let field_value = match fields_map.get("loadBalancerListenerPort") {
                            Some(value) => value,
                            None => bail!("Missing field 'loadBalancerListenerPort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_target_group: {
                        let field_value = match fields_map.get("loadBalancerTargetGroup") {
                            Some(value) => value,
                            None => bail!("Missing field 'loadBalancerTargetGroup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_target_groups: {
                        let field_value = match fields_map.get("loadBalancerTargetGroups") {
                            Some(value) => value,
                            None => bail!("Missing field 'loadBalancerTargetGroups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_target_port: {
                        let field_value = match fields_map.get("loadBalancerTargetPort") {
                            Some(value) => value,
                            None => bail!("Missing field 'loadBalancerTargetPort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#missing_component: {
                        let field_value = match fields_map.get("missingComponent") {
                            Some(value) => value,
                            None => bail!("Missing field 'missingComponent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nat_gateways: {
                        let field_value = match fields_map.get("natGateways") {
                            Some(value) => value,
                            None => bail!("Missing field 'natGateways' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_interfaces: {
                        let field_value = match fields_map.get("networkInterfaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkInterfaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#packet_field: {
                        let field_value = match fields_map.get("packetField") {
                            Some(value) => value,
                            None => bail!("Missing field 'packetField' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port_ranges: {
                        let field_value = match fields_map.get("portRanges") {
                            Some(value) => value,
                            None => bail!("Missing field 'portRanges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_lists: {
                        let field_value = match fields_map.get("prefixLists") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefixLists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocols: {
                        let field_value = match fields_map.get("protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_table_routes: {
                        let field_value = match fields_map.get("routeTableRoutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'routeTableRoutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_tables: {
                        let field_value = match fields_map.get("routeTables") {
                            Some(value) => value,
                            None => bail!("Missing field 'routeTables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_group: {
                        let field_value = match fields_map.get("securityGroup") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityGroup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_group_rules: {
                        let field_value = match fields_map.get("securityGroupRules") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityGroupRules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_groups: {
                        let field_value = match fields_map.get("securityGroups") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityGroups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_vpcs: {
                        let field_value = match fields_map.get("sourceVpcs") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceVpcs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_route_tables: {
                        let field_value = match fields_map.get("subnetRouteTables") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetRouteTables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnets: {
                        let field_value = match fields_map.get("subnets") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transit_gateway_attachments: {
                        let field_value = match fields_map.get("transitGatewayAttachments") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitGatewayAttachments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transit_gateway_route_table_routes: {
                        let field_value = match fields_map.get("transitGatewayRouteTableRoutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitGatewayRouteTableRoutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transit_gateway_route_tables: {
                        let field_value = match fields_map.get("transitGatewayRouteTables") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitGatewayRouteTables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transit_gateways: {
                        let field_value = match fields_map.get("transitGateways") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitGateways' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_endpoints: {
                        let field_value = match fields_map.get("vpcEndpoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpcEndpoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_peering_connections: {
                        let field_value = match fields_map.get("vpcPeeringConnections") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpcPeeringConnections' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpcs: {
                        let field_value = match fields_map.get("vpcs") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpcs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpn_connections: {
                        let field_value = match fields_map.get("vpnConnections") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpnConnections' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpn_gateways: {
                        let field_value = match fields_map.get("vpnGateways") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpnGateways' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
