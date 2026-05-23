#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkInsightsAnalysisReturnPathComponentRouteTableRoute {
    #[builder(into)]
    pub r#destination_cidr: String,
    #[builder(into)]
    pub r#destination_prefix_list_id: String,
    #[builder(into)]
    pub r#egress_only_internet_gateway_id: String,
    #[builder(into)]
    pub r#gateway_id: String,
    #[builder(into)]
    pub r#instance_id: String,
    #[builder(into)]
    pub r#nat_gateway_id: String,
    #[builder(into)]
    pub r#network_interface_id: String,
    #[builder(into)]
    pub r#origin: String,
    #[builder(into)]
    pub r#transit_gateway_id: String,
    #[builder(into)]
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
                    "destinationCidr",
                    &self.r#destination_cidr,
                ),
                to_pulumi_object_field(
                    "destinationPrefixListId",
                    &self.r#destination_prefix_list_id,
                ),
                to_pulumi_object_field(
                    "egressOnlyInternetGatewayId",
                    &self.r#egress_only_internet_gateway_id,
                ),
                to_pulumi_object_field(
                    "gatewayId",
                    &self.r#gateway_id,
                ),
                to_pulumi_object_field(
                    "instanceId",
                    &self.r#instance_id,
                ),
                to_pulumi_object_field(
                    "natGatewayId",
                    &self.r#nat_gateway_id,
                ),
                to_pulumi_object_field(
                    "networkInterfaceId",
                    &self.r#network_interface_id,
                ),
                to_pulumi_object_field(
                    "origin",
                    &self.r#origin,
                ),
                to_pulumi_object_field(
                    "transitGatewayId",
                    &self.r#transit_gateway_id,
                ),
                to_pulumi_object_field(
                    "vpcPeeringConnectionId",
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
                        let field_value = match fields_map.get("destinationCidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationCidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_prefix_list_id: {
                        let field_value = match fields_map.get("destinationPrefixListId") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationPrefixListId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#egress_only_internet_gateway_id: {
                        let field_value = match fields_map.get("egressOnlyInternetGatewayId") {
                            Some(value) => value,
                            None => bail!("Missing field 'egressOnlyInternetGatewayId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gateway_id: {
                        let field_value = match fields_map.get("gatewayId") {
                            Some(value) => value,
                            None => bail!("Missing field 'gatewayId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_id: {
                        let field_value = match fields_map.get("instanceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nat_gateway_id: {
                        let field_value = match fields_map.get("natGatewayId") {
                            Some(value) => value,
                            None => bail!("Missing field 'natGatewayId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_interface_id: {
                        let field_value = match fields_map.get("networkInterfaceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkInterfaceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("transitGatewayId") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitGatewayId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_peering_connection_id: {
                        let field_value = match fields_map.get("vpcPeeringConnectionId") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpcPeeringConnectionId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
