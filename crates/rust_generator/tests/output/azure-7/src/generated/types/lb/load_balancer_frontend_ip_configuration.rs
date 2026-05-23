#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LoadBalancerFrontendIpConfiguration {
    /// The Frontend IP Configuration ID of a Gateway SKU Load Balancer.
    #[builder(into)]
    pub r#gateway_load_balancer_frontend_ip_configuration_id: Option<String>,
    /// The id of the Frontend IP Configuration.
    #[builder(into)]
    pub r#id: Option<String>,
    /// The list of IDs of inbound rules that use this frontend IP.
    #[builder(into)]
    pub r#inbound_nat_rules: Option<Vec<String>>,
    /// The list of IDs of load balancing rules that use this frontend IP.
    #[builder(into)]
    pub r#load_balancer_rules: Option<Vec<String>>,
    /// Specifies the name of the frontend IP configuration.
    #[builder(into)]
    pub r#name: String,
    /// The list of IDs outbound rules that use this frontend IP.
    #[builder(into)]
    pub r#outbound_rules: Option<Vec<String>>,
    /// Private IP Address to assign to the Load Balancer. The last one and first four IPs in any range are reserved and cannot be manually assigned.
    #[builder(into)]
    pub r#private_ip_address: Option<String>,
    /// The allocation method for the Private IP Address used by this Load Balancer. Possible values as `Dynamic` and `Static`.
    #[builder(into)]
    pub r#private_ip_address_allocation: Option<String>,
    /// The version of IP that the Private IP Address is. Possible values are `IPv4` or `IPv6`.
    #[builder(into)]
    pub r#private_ip_address_version: Option<String>,
    /// The ID of a Public IP Address which should be associated with the Load Balancer.
    #[builder(into)]
    pub r#public_ip_address_id: Option<String>,
    /// The ID of a Public IP Prefix which should be associated with the Load Balancer. Public IP Prefix can only be used with outbound rules.
    #[builder(into)]
    pub r#public_ip_prefix_id: Option<String>,
    /// The ID of the Subnet which should be associated with the IP Configuration.
    #[builder(into)]
    pub r#subnet_id: Option<String>,
    /// Specifies a list of Availability Zones in which the IP Address for this Load Balancer should be located.
    /// 
    /// > **NOTE:** Availability Zones are only supported with a [Standard SKU](https://docs.microsoft.com/azure/load-balancer/load-balancer-standard-availability-zones) and [in select regions](https://docs.microsoft.com/azure/availability-zones/az-overview) at this time.
    #[builder(into)]
    pub r#zones: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LoadBalancerFrontendIpConfiguration {
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
                    "gatewayLoadBalancerFrontendIpConfigurationId",
                    &self.r#gateway_load_balancer_frontend_ip_configuration_id,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "inboundNatRules",
                    &self.r#inbound_nat_rules,
                ),
                to_pulumi_object_field(
                    "loadBalancerRules",
                    &self.r#load_balancer_rules,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "outboundRules",
                    &self.r#outbound_rules,
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
                    "publicIpPrefixId",
                    &self.r#public_ip_prefix_id,
                ),
                to_pulumi_object_field(
                    "subnetId",
                    &self.r#subnet_id,
                ),
                to_pulumi_object_field(
                    "zones",
                    &self.r#zones,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LoadBalancerFrontendIpConfiguration {
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
                    r#gateway_load_balancer_frontend_ip_configuration_id: {
                        let field_value = match fields_map.get("gatewayLoadBalancerFrontendIpConfigurationId") {
                            Some(value) => value,
                            None => bail!("Missing field 'gatewayLoadBalancerFrontendIpConfigurationId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inbound_nat_rules: {
                        let field_value = match fields_map.get("inboundNatRules") {
                            Some(value) => value,
                            None => bail!("Missing field 'inboundNatRules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_rules: {
                        let field_value = match fields_map.get("loadBalancerRules") {
                            Some(value) => value,
                            None => bail!("Missing field 'loadBalancerRules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#outbound_rules: {
                        let field_value = match fields_map.get("outboundRules") {
                            Some(value) => value,
                            None => bail!("Missing field 'outboundRules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#public_ip_prefix_id: {
                        let field_value = match fields_map.get("publicIpPrefixId") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicIpPrefixId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#zones: {
                        let field_value = match fields_map.get("zones") {
                            Some(value) => value,
                            None => bail!("Missing field 'zones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
