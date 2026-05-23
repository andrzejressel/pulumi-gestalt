#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkInterfaceIpConfiguration {
    /// A list of Backend Address Pool IDs within a Application Gateway that this Network Interface is connected to.
    #[builder(into)]
    #[serde(rename = "applicationGatewayBackendAddressPoolsIds")]
    pub r#application_gateway_backend_address_pools_ids: Vec<String>,
    #[builder(into)]
    #[serde(rename = "applicationSecurityGroupIds")]
    pub r#application_security_group_ids: Vec<String>,
    /// The Frontend IP Configuration ID of a Gateway SKU Load Balancer the Network Interface is consuming.
    #[builder(into)]
    #[serde(rename = "gatewayLoadBalancerFrontendIpConfigurationId")]
    pub r#gateway_load_balancer_frontend_ip_configuration_id: String,
    /// A list of Backend Address Pool IDs within a Load Balancer that this Network Interface is connected to.
    #[builder(into)]
    #[serde(rename = "loadBalancerBackendAddressPoolsIds")]
    pub r#load_balancer_backend_address_pools_ids: Vec<String>,
    /// A list of Inbound NAT Rule IDs within a Load Balancer that this Network Interface is connected to.
    #[builder(into)]
    #[serde(rename = "loadBalancerInboundNatRulesIds")]
    pub r#load_balancer_inbound_nat_rules_ids: Vec<String>,
    /// Specifies the name of the Network Interface.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// is this the Primary IP Configuration for this Network Interface?
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: bool,
    /// The Private IP Address assigned to this Network Interface.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: String,
    /// The IP Address allocation type for the Private address, such as `Dynamic` or `Static`.
    #[builder(into)]
    #[serde(rename = "privateIpAddressAllocation")]
    pub r#private_ip_address_allocation: String,
    #[builder(into)]
    #[serde(rename = "privateIpAddressVersion")]
    pub r#private_ip_address_version: String,
    /// The ID of the Public IP Address which is connected to this Network Interface.
    #[builder(into)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: String,
    /// The ID of the Subnet which the Network Interface is connected to.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNetworkInterfaceIpConfiguration {
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
                    "applicationGatewayBackendAddressPoolsIds",
                    &self.r#application_gateway_backend_address_pools_ids,
                ),
                to_pulumi_object_field(
                    "applicationSecurityGroupIds",
                    &self.r#application_security_group_ids,
                ),
                to_pulumi_object_field(
                    "gatewayLoadBalancerFrontendIpConfigurationId",
                    &self.r#gateway_load_balancer_frontend_ip_configuration_id,
                ),
                to_pulumi_object_field(
                    "loadBalancerBackendAddressPoolsIds",
                    &self.r#load_balancer_backend_address_pools_ids,
                ),
                to_pulumi_object_field(
                    "loadBalancerInboundNatRulesIds",
                    &self.r#load_balancer_inbound_nat_rules_ids,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "primary",
                    &self.r#primary,
                ),
                to_pulumi_object_field(
                    "privateIpAddress",
                    &self.r#private_ip_address,
                ),
                to_pulumi_object_field(
                    "privateIpAddressAllocation",
                    &self.r#private_ip_address_allocation,
                ),
                to_pulumi_object_field(
                    "privateIpAddressVersion",
                    &self.r#private_ip_address_version,
                ),
                to_pulumi_object_field(
                    "publicIpAddressId",
                    &self.r#public_ip_address_id,
                ),
                to_pulumi_object_field(
                    "subnetId",
                    &self.r#subnet_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetNetworkInterfaceIpConfiguration {
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
                    r#application_gateway_backend_address_pools_ids: {
                        let field_value = match fields_map.get("applicationGatewayBackendAddressPoolsIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationGatewayBackendAddressPoolsIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_security_group_ids: {
                        let field_value = match fields_map.get("applicationSecurityGroupIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationSecurityGroupIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gateway_load_balancer_frontend_ip_configuration_id: {
                        let field_value = match fields_map.get("gatewayLoadBalancerFrontendIpConfigurationId") {
                            Some(value) => value,
                            None => bail!("Missing field 'gatewayLoadBalancerFrontendIpConfigurationId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_backend_address_pools_ids: {
                        let field_value = match fields_map.get("loadBalancerBackendAddressPoolsIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'loadBalancerBackendAddressPoolsIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_inbound_nat_rules_ids: {
                        let field_value = match fields_map.get("loadBalancerInboundNatRulesIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'loadBalancerInboundNatRulesIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary: {
                        let field_value = match fields_map.get("primary") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip_address: {
                        let field_value = match fields_map.get("privateIpAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateIpAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip_address_allocation: {
                        let field_value = match fields_map.get("privateIpAddressAllocation") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateIpAddressAllocation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip_address_version: {
                        let field_value = match fields_map.get("privateIpAddressVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateIpAddressVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_address_id: {
                        let field_value = match fields_map.get("publicIpAddressId") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicIpAddressId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
