#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DefaultRouteTableRoute {
    /// The CIDR block of the route.
    #[builder(into)]
    pub r#cidr_block: Option<String>,
    /// The Amazon Resource Name (ARN) of a core network.
    #[builder(into)]
    pub r#core_network_arn: Option<String>,
    /// The ID of a managed prefix list destination of the route.
    /// 
    /// One of the following target arguments must be supplied:
    #[builder(into)]
    pub r#destination_prefix_list_id: Option<String>,
    /// Identifier of a VPC Egress Only Internet Gateway.
    #[builder(into)]
    pub r#egress_only_gateway_id: Option<String>,
    /// Identifier of a VPC internet gateway or a virtual private gateway.
    #[builder(into)]
    pub r#gateway_id: Option<String>,
    /// Identifier of an EC2 instance.
    #[builder(into)]
    pub r#instance_id: Option<String>,
    /// The Ipv6 CIDR block of the route
    #[builder(into)]
    pub r#ipv_6_cidr_block: Option<String>,
    /// Identifier of a VPC NAT gateway.
    #[builder(into)]
    pub r#nat_gateway_id: Option<String>,
    /// Identifier of an EC2 network interface.
    #[builder(into)]
    pub r#network_interface_id: Option<String>,
    /// Identifier of an EC2 Transit Gateway.
    #[builder(into)]
    pub r#transit_gateway_id: Option<String>,
    /// Identifier of a VPC Endpoint. This route must be removed prior to VPC Endpoint deletion.
    #[builder(into)]
    pub r#vpc_endpoint_id: Option<String>,
    /// Identifier of a VPC peering connection.
    /// 
    /// Note that the default route, mapping the VPC's CIDR block to "local", is created implicitly and cannot be specified.
    #[builder(into)]
    pub r#vpc_peering_connection_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DefaultRouteTableRoute {
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
                    "cidrBlock",
                    &self.r#cidr_block,
                ),
                to_pulumi_object_field(
                    "coreNetworkArn",
                    &self.r#core_network_arn,
                ),
                to_pulumi_object_field(
                    "destinationPrefixListId",
                    &self.r#destination_prefix_list_id,
                ),
                to_pulumi_object_field(
                    "egressOnlyGatewayId",
                    &self.r#egress_only_gateway_id,
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
                    "ipv6CidrBlock",
                    &self.r#ipv_6_cidr_block,
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
                    "transitGatewayId",
                    &self.r#transit_gateway_id,
                ),
                to_pulumi_object_field(
                    "vpcEndpointId",
                    &self.r#vpc_endpoint_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DefaultRouteTableRoute {
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
                    r#cidr_block: {
                        let field_value = match fields_map.get("cidrBlock") {
                            Some(value) => value,
                            None => bail!("Missing field 'cidrBlock' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#core_network_arn: {
                        let field_value = match fields_map.get("coreNetworkArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'coreNetworkArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#egress_only_gateway_id: {
                        let field_value = match fields_map.get("egressOnlyGatewayId") {
                            Some(value) => value,
                            None => bail!("Missing field 'egressOnlyGatewayId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#ipv_6_cidr_block: {
                        let field_value = match fields_map.get("ipv6CidrBlock") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv6CidrBlock' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#transit_gateway_id: {
                        let field_value = match fields_map.get("transitGatewayId") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitGatewayId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_endpoint_id: {
                        let field_value = match fields_map.get("vpcEndpointId") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpcEndpointId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
