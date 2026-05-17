#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetOrchestratedVirtualMachineScaleSetNetworkInterface {
    /// Is accelerated networking enabled?
    #[builder(into)]
    #[serde(rename = "acceleratedNetworkingEnabled")]
    pub r#accelerated_networking_enabled: bool,
    /// An array of the DNS servers in use.
    #[builder(into)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Vec<String>,
    /// An `ip_configuration` block as documented below.
    #[builder(into)]
    #[serde(rename = "ipConfigurations")]
    pub r#ip_configurations: Vec<super::super::types::compute::GetOrchestratedVirtualMachineScaleSetNetworkInterfaceIpConfiguration>,
    /// Is IP forwarding enabled?
    #[builder(into)]
    #[serde(rename = "ipForwardingEnabled")]
    pub r#ip_forwarding_enabled: bool,
    /// The name of this Orchestrated Virtual Machine Scale Set.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The identifier for the network security group.
    #[builder(into)]
    #[serde(rename = "networkSecurityGroupId")]
    pub r#network_security_group_id: String,
    /// If this ip_configuration is the primary one.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetOrchestratedVirtualMachineScaleSetNetworkInterface {
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
                "accelerated_networking_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#accelerated_networking_enabled,
                )
                .await,
            );
            map.insert(
                "dns_servers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dns_servers,
                )
                .await,
            );
            map.insert(
                "ip_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_configurations,
                )
                .await,
            );
            map.insert(
                "ip_forwarding_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_forwarding_enabled,
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
                "network_security_group_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_security_group_id,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetOrchestratedVirtualMachineScaleSetNetworkInterface {
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
                    r#accelerated_networking_enabled: {
                        let field_value = match fields_map.get("accelerated_networking_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerated_networking_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_servers: {
                        let field_value = match fields_map.get("dns_servers") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_servers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_configurations: {
                        let field_value = match fields_map.get("ip_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_forwarding_enabled: {
                        let field_value = match fields_map.get("ip_forwarding_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_forwarding_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#network_security_group_id: {
                        let field_value = match fields_map.get("network_security_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_security_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
