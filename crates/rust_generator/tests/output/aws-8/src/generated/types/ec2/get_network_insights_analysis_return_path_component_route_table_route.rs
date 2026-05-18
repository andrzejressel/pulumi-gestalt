#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkInsightsAnalysisReturnPathComponentRouteTableRoute {
    #[builder(into)]
    #[serde(rename = "destinationCidr")]
    pub r#destination_cidr: String,
    #[builder(into)]
    #[serde(rename = "destinationPrefixListId")]
    pub r#destination_prefix_list_id: String,
    #[builder(into)]
    #[serde(rename = "egressOnlyInternetGatewayId")]
    pub r#egress_only_internet_gateway_id: String,
    #[builder(into)]
    #[serde(rename = "gatewayId")]
    pub r#gateway_id: String,
    #[builder(into)]
    #[serde(rename = "instanceId")]
    pub r#instance_id: String,
    #[builder(into)]
    #[serde(rename = "natGatewayId")]
    pub r#nat_gateway_id: String,
    #[builder(into)]
    #[serde(rename = "networkInterfaceId")]
    pub r#network_interface_id: String,
    #[builder(into)]
    #[serde(rename = "origin")]
    pub r#origin: String,
    #[builder(into)]
    #[serde(rename = "transitGatewayId")]
    pub r#transit_gateway_id: String,
    #[builder(into)]
    #[serde(rename = "vpcPeeringConnectionId")]
    pub r#vpc_peering_connection_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNetworkInsightsAnalysisReturnPathComponentRouteTableRoute {
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
                    "destination_cidr",
                    &self.r#destination_cidr,
                ),
                to_pulumi_object_field(
                    "destination_prefix_list_id",
                    &self.r#destination_prefix_list_id,
                ),
                to_pulumi_object_field(
                    "egress_only_internet_gateway_id",
                    &self.r#egress_only_internet_gateway_id,
                ),
                to_pulumi_object_field(
                    "gateway_id",
                    &self.r#gateway_id,
                ),
                to_pulumi_object_field(
                    "instance_id",
                    &self.r#instance_id,
                ),
                to_pulumi_object_field(
                    "nat_gateway_id",
                    &self.r#nat_gateway_id,
                ),
                to_pulumi_object_field(
                    "network_interface_id",
                    &self.r#network_interface_id,
                ),
                to_pulumi_object_field(
                    "origin",
                    &self.r#origin,
                ),
                to_pulumi_object_field(
                    "transit_gateway_id",
                    &self.r#transit_gateway_id,
                ),
                to_pulumi_object_field(
                    "vpc_peering_connection_id",
                    &self.r#vpc_peering_connection_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetNetworkInsightsAnalysisReturnPathComponentRouteTableRoute {
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
                    r#destination_cidr: {
                        let field_value = match fields_map.get("destination_cidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_cidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_prefix_list_id: {
                        let field_value = match fields_map.get("destination_prefix_list_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_prefix_list_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#egress_only_internet_gateway_id: {
                        let field_value = match fields_map.get("egress_only_internet_gateway_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'egress_only_internet_gateway_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gateway_id: {
                        let field_value = match fields_map.get("gateway_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'gateway_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_id: {
                        let field_value = match fields_map.get("instance_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nat_gateway_id: {
                        let field_value = match fields_map.get("nat_gateway_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'nat_gateway_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_interface_id: {
                        let field_value = match fields_map.get("network_interface_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_interface_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin: {
                        let field_value = match fields_map.get("origin") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transit_gateway_id: {
                        let field_value = match fields_map.get("transit_gateway_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'transit_gateway_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_peering_connection_id: {
                        let field_value = match fields_map.get("vpc_peering_connection_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_peering_connection_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
