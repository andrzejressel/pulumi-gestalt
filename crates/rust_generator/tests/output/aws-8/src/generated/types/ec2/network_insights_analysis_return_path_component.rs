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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkInsightsAnalysisReturnPathComponent {
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
                    "acl_rules",
                    &self.r#acl_rules,
                ),
                to_pulumi_object_field(
                    "additional_details",
                    &self.r#additional_details,
                ),
                to_pulumi_object_field(
                    "attached_tos",
                    &self.r#attached_tos,
                ),
                to_pulumi_object_field(
                    "components",
                    &self.r#components,
                ),
                to_pulumi_object_field(
                    "destination_vpcs",
                    &self.r#destination_vpcs,
                ),
                to_pulumi_object_field(
                    "inbound_headers",
                    &self.r#inbound_headers,
                ),
                to_pulumi_object_field(
                    "outbound_headers",
                    &self.r#outbound_headers,
                ),
                to_pulumi_object_field(
                    "route_table_routes",
                    &self.r#route_table_routes,
                ),
                to_pulumi_object_field(
                    "security_group_rules",
                    &self.r#security_group_rules,
                ),
                to_pulumi_object_field(
                    "sequence_number",
                    &self.r#sequence_number,
                ),
                to_pulumi_object_field(
                    "source_vpcs",
                    &self.r#source_vpcs,
                ),
                to_pulumi_object_field(
                    "subnets",
                    &self.r#subnets,
                ),
                to_pulumi_object_field(
                    "transit_gateway_route_table_routes",
                    &self.r#transit_gateway_route_table_routes,
                ),
                to_pulumi_object_field(
                    "transit_gateways",
                    &self.r#transit_gateways,
                ),
                to_pulumi_object_field(
                    "vpcs",
                    &self.r#vpcs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkInsightsAnalysisReturnPathComponent {
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
