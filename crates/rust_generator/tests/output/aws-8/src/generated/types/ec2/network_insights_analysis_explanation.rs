#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkInsightsAnalysisExplanation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "acl_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#acl_rules,
                )
                .await,
            );
            map.insert(
                "acls".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#acls,
                )
                .await,
            );
            map.insert(
                "address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#address,
                )
                .await,
            );
            map.insert(
                "addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#addresses,
                )
                .await,
            );
            map.insert(
                "attached_tos".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#attached_tos,
                )
                .await,
            );
            map.insert(
                "availability_zones".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#availability_zones,
                )
                .await,
            );
            map.insert(
                "cidrs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cidrs,
                )
                .await,
            );
            map.insert(
                "classic_load_balancer_listeners".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#classic_load_balancer_listeners,
                )
                .await,
            );
            map.insert(
                "components".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#components,
                )
                .await,
            );
            map.insert(
                "customer_gateways".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#customer_gateways,
                )
                .await,
            );
            map.insert(
                "destination_vpcs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination_vpcs,
                )
                .await,
            );
            map.insert(
                "destinations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destinations,
                )
                .await,
            );
            map.insert(
                "direction".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#direction,
                )
                .await,
            );
            map.insert(
                "elastic_load_balancer_listeners".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#elastic_load_balancer_listeners,
                )
                .await,
            );
            map.insert(
                "explanation_code".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#explanation_code,
                )
                .await,
            );
            map.insert(
                "ingress_route_tables".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ingress_route_tables,
                )
                .await,
            );
            map.insert(
                "internet_gateways".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#internet_gateways,
                )
                .await,
            );
            map.insert(
                "load_balancer_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#load_balancer_arn,
                )
                .await,
            );
            map.insert(
                "load_balancer_listener_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#load_balancer_listener_port,
                )
                .await,
            );
            map.insert(
                "load_balancer_target_group".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#load_balancer_target_group,
                )
                .await,
            );
            map.insert(
                "load_balancer_target_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#load_balancer_target_groups,
                )
                .await,
            );
            map.insert(
                "load_balancer_target_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#load_balancer_target_port,
                )
                .await,
            );
            map.insert(
                "missing_component".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#missing_component,
                )
                .await,
            );
            map.insert(
                "nat_gateways".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nat_gateways,
                )
                .await,
            );
            map.insert(
                "network_interfaces".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_interfaces,
                )
                .await,
            );
            map.insert(
                "packet_field".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#packet_field,
                )
                .await,
            );
            map.insert(
                "port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#port,
                )
                .await,
            );
            map.insert(
                "port_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#port_ranges,
                )
                .await,
            );
            map.insert(
                "prefix_lists".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#prefix_lists,
                )
                .await,
            );
            map.insert(
                "protocols".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protocols,
                )
                .await,
            );
            map.insert(
                "route_table_routes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#route_table_routes,
                )
                .await,
            );
            map.insert(
                "route_tables".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#route_tables,
                )
                .await,
            );
            map.insert(
                "security_group".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_group,
                )
                .await,
            );
            map.insert(
                "security_group_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_group_rules,
                )
                .await,
            );
            map.insert(
                "security_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_groups,
                )
                .await,
            );
            map.insert(
                "source_vpcs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_vpcs,
                )
                .await,
            );
            map.insert(
                "state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#state,
                )
                .await,
            );
            map.insert(
                "subnet_route_tables".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnet_route_tables,
                )
                .await,
            );
            map.insert(
                "subnets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnets,
                )
                .await,
            );
            map.insert(
                "transit_gateway_attachments".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transit_gateway_attachments,
                )
                .await,
            );
            map.insert(
                "transit_gateway_route_table_routes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transit_gateway_route_table_routes,
                )
                .await,
            );
            map.insert(
                "transit_gateway_route_tables".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transit_gateway_route_tables,
                )
                .await,
            );
            map.insert(
                "transit_gateways".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transit_gateways,
                )
                .await,
            );
            map.insert(
                "vpc_endpoints".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vpc_endpoints,
                )
                .await,
            );
            map.insert(
                "vpc_peering_connections".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vpc_peering_connections,
                )
                .await,
            );
            map.insert(
                "vpcs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vpcs,
                )
                .await,
            );
            map.insert(
                "vpn_connections".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vpn_connections,
                )
                .await,
            );
            map.insert(
                "vpn_gateways".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vpn_gateways,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkInsightsAnalysisExplanation {
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
                        let field_value = match fields_map.get("acl_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'acl_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("attached_tos") {
                            Some(value) => value,
                            None => bail!("Missing field 'attached_tos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#availability_zones: {
                        let field_value = match fields_map.get("availability_zones") {
                            Some(value) => value,
                            None => bail!("Missing field 'availability_zones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("classic_load_balancer_listeners") {
                            Some(value) => value,
                            None => bail!("Missing field 'classic_load_balancer_listeners' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("customer_gateways") {
                            Some(value) => value,
                            None => bail!("Missing field 'customer_gateways' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_vpcs: {
                        let field_value = match fields_map.get("destination_vpcs") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_vpcs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("elastic_load_balancer_listeners") {
                            Some(value) => value,
                            None => bail!("Missing field 'elastic_load_balancer_listeners' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#explanation_code: {
                        let field_value = match fields_map.get("explanation_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'explanation_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ingress_route_tables: {
                        let field_value = match fields_map.get("ingress_route_tables") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingress_route_tables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#internet_gateways: {
                        let field_value = match fields_map.get("internet_gateways") {
                            Some(value) => value,
                            None => bail!("Missing field 'internet_gateways' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_arn: {
                        let field_value = match fields_map.get("load_balancer_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancer_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_listener_port: {
                        let field_value = match fields_map.get("load_balancer_listener_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancer_listener_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_target_group: {
                        let field_value = match fields_map.get("load_balancer_target_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancer_target_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_target_groups: {
                        let field_value = match fields_map.get("load_balancer_target_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancer_target_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_target_port: {
                        let field_value = match fields_map.get("load_balancer_target_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancer_target_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#missing_component: {
                        let field_value = match fields_map.get("missing_component") {
                            Some(value) => value,
                            None => bail!("Missing field 'missing_component' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nat_gateways: {
                        let field_value = match fields_map.get("nat_gateways") {
                            Some(value) => value,
                            None => bail!("Missing field 'nat_gateways' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_interfaces: {
                        let field_value = match fields_map.get("network_interfaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_interfaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#packet_field: {
                        let field_value = match fields_map.get("packet_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'packet_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("port_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'port_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_lists: {
                        let field_value = match fields_map.get("prefix_lists") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix_lists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("route_table_routes") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_table_routes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_tables: {
                        let field_value = match fields_map.get("route_tables") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_tables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_group: {
                        let field_value = match fields_map.get("security_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_group_rules: {
                        let field_value = match fields_map.get("security_group_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_group_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_groups: {
                        let field_value = match fields_map.get("security_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_vpcs: {
                        let field_value = match fields_map.get("source_vpcs") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_vpcs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("subnet_route_tables") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_route_tables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("transit_gateway_attachments") {
                            Some(value) => value,
                            None => bail!("Missing field 'transit_gateway_attachments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transit_gateway_route_table_routes: {
                        let field_value = match fields_map.get("transit_gateway_route_table_routes") {
                            Some(value) => value,
                            None => bail!("Missing field 'transit_gateway_route_table_routes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transit_gateway_route_tables: {
                        let field_value = match fields_map.get("transit_gateway_route_tables") {
                            Some(value) => value,
                            None => bail!("Missing field 'transit_gateway_route_tables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transit_gateways: {
                        let field_value = match fields_map.get("transit_gateways") {
                            Some(value) => value,
                            None => bail!("Missing field 'transit_gateways' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_endpoints: {
                        let field_value = match fields_map.get("vpc_endpoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_endpoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_peering_connections: {
                        let field_value = match fields_map.get("vpc_peering_connections") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_peering_connections' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("vpn_connections") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpn_connections' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpn_gateways: {
                        let field_value = match fields_map.get("vpn_gateways") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpn_gateways' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
