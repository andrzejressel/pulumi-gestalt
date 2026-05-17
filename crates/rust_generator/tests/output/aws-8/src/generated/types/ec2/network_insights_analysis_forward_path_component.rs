#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkInsightsAnalysisForwardPathComponent {
    #[builder(into)]
    #[serde(rename = "aclRules")]
    pub r#acl_rules: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentAclRule>>,
    #[builder(into)]
    #[serde(rename = "additionalDetails")]
    pub r#additional_details: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentAdditionalDetail>>,
    #[builder(into)]
    #[serde(rename = "attachedTos")]
    pub r#attached_tos: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentAttachedTo>>,
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentComponent>>,
    #[builder(into)]
    #[serde(rename = "destinationVpcs")]
    pub r#destination_vpcs: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentDestinationVpc>>,
    #[builder(into)]
    #[serde(rename = "inboundHeaders")]
    pub r#inbound_headers: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentInboundHeader>>,
    #[builder(into)]
    #[serde(rename = "outboundHeaders")]
    pub r#outbound_headers: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentOutboundHeader>>,
    #[builder(into)]
    #[serde(rename = "routeTableRoutes")]
    pub r#route_table_routes: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentRouteTableRoute>>,
    #[builder(into)]
    #[serde(rename = "securityGroupRules")]
    pub r#security_group_rules: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentSecurityGroupRule>>,
    #[builder(into)]
    #[serde(rename = "sequenceNumber")]
    pub r#sequence_number: Option<i32>,
    #[builder(into)]
    #[serde(rename = "sourceVpcs")]
    pub r#source_vpcs: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentSourceVpc>>,
    #[builder(into)]
    #[serde(rename = "subnets")]
    pub r#subnets: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentSubnet>>,
    #[builder(into)]
    #[serde(rename = "transitGatewayRouteTableRoutes")]
    pub r#transit_gateway_route_table_routes: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentTransitGatewayRouteTableRoute>>,
    #[builder(into)]
    #[serde(rename = "transitGateways")]
    pub r#transit_gateways: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentTransitGateway>>,
    #[builder(into)]
    #[serde(rename = "vpcs")]
    pub r#vpcs: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentVpc>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkInsightsAnalysisForwardPathComponent {
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
                "additional_details".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#additional_details,
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
                "components".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#components,
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
                "inbound_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#inbound_headers,
                )
                .await,
            );
            map.insert(
                "outbound_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#outbound_headers,
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
                "security_group_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_group_rules,
                )
                .await,
            );
            map.insert(
                "sequence_number".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sequence_number,
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
                "subnets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnets,
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
                "transit_gateways".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transit_gateways,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkInsightsAnalysisForwardPathComponent {
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
                    r#additional_details: {
                        let field_value = match fields_map.get("additional_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#components: {
                        let field_value = match fields_map.get("components") {
                            Some(value) => value,
                            None => bail!("Missing field 'components' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#inbound_headers: {
                        let field_value = match fields_map.get("inbound_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'inbound_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outbound_headers: {
                        let field_value = match fields_map.get("outbound_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'outbound_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#security_group_rules: {
                        let field_value = match fields_map.get("security_group_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_group_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sequence_number: {
                        let field_value = match fields_map.get("sequence_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'sequence_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#subnets: {
                        let field_value = match fields_map.get("subnets") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#transit_gateways: {
                        let field_value = match fields_map.get("transit_gateways") {
                            Some(value) => value,
                            None => bail!("Missing field 'transit_gateways' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
