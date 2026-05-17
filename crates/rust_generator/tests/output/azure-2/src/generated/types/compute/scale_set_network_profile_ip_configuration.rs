#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScaleSetNetworkProfileIpConfiguration {
    /// Specifies an array of references to backend address pools of application gateways. A scale set can reference backend address pools of multiple application gateways. Multiple scale sets can use the same application gateway.
    #[builder(into)]
    #[serde(rename = "applicationGatewayBackendAddressPoolIds")]
    pub r#application_gateway_backend_address_pool_ids: Option<Vec<String>>,
    /// Specifies up to `20` application security group IDs.
    #[builder(into)]
    #[serde(rename = "applicationSecurityGroupIds")]
    pub r#application_security_group_ids: Option<Vec<String>>,
    /// Specifies an array of references to backend address pools of load balancers. A scale set can reference backend address pools of one public and one internal load balancer. Multiple scale sets cannot use the same load balancer.
    /// 
    /// > **NOTE:** When using this field you'll also need to configure a Rule for the Load Balancer, and use a `depends_on` between this resource and the Load Balancer Rule.
    #[builder(into)]
    #[serde(rename = "loadBalancerBackendAddressPoolIds")]
    pub r#load_balancer_backend_address_pool_ids: Option<Vec<String>>,
    /// Specifies an array of references to inbound NAT pools for load balancers. A scale set can reference inbound NAT pools of one public and one internal load balancer. Multiple scale sets cannot use the same load balancer.
    /// 
    /// > **NOTE:** When using this field you'll also need to configure a Rule for the Load Balancer, and use a `depends_on` between this resource and the Load Balancer Rule.
    #[builder(into)]
    #[serde(rename = "loadBalancerInboundNatRulesIds")]
    pub r#load_balancer_inbound_nat_rules_ids: Option<Vec<String>>,
    /// Specifies name of the IP configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies if this ip_configuration is the primary one.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: bool,
    /// Describes a virtual machines scale set IP Configuration's PublicIPAddress configuration. The `public_ip_address_configuration` block is documented below.
    #[builder(into)]
    #[serde(rename = "publicIpAddressConfiguration")]
    pub r#public_ip_address_configuration: Option<Box<super::super::types::compute::ScaleSetNetworkProfileIpConfigurationPublicIpAddressConfiguration>>,
    /// Specifies the identifier of the subnet.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScaleSetNetworkProfileIpConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "application_gateway_backend_address_pool_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#application_gateway_backend_address_pool_ids,
                )
                .await,
            );
            map.insert(
                "application_security_group_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#application_security_group_ids,
                )
                .await,
            );
            map.insert(
                "load_balancer_backend_address_pool_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#load_balancer_backend_address_pool_ids,
                )
                .await,
            );
            map.insert(
                "load_balancer_inbound_nat_rules_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#load_balancer_inbound_nat_rules_ids,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "primary".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#primary,
                )
                .await,
            );
            map.insert(
                "public_ip_address_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_ip_address_configuration,
                )
                .await,
            );
            map.insert(
                "subnet_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnet_id,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScaleSetNetworkProfileIpConfiguration {
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
                    r#application_gateway_backend_address_pool_ids: {
                        let field_value = match fields_map.get("application_gateway_backend_address_pool_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_gateway_backend_address_pool_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_security_group_ids: {
                        let field_value = match fields_map.get("application_security_group_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_security_group_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_backend_address_pool_ids: {
                        let field_value = match fields_map.get("load_balancer_backend_address_pool_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancer_backend_address_pool_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_inbound_nat_rules_ids: {
                        let field_value = match fields_map.get("load_balancer_inbound_nat_rules_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancer_inbound_nat_rules_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#public_ip_address_configuration: {
                        let field_value = match fields_map.get("public_ip_address_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_ip_address_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
