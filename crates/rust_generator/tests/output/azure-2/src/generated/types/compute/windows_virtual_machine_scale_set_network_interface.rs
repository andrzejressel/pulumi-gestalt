#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsVirtualMachineScaleSetNetworkInterface {
    /// A list of IP Addresses of DNS Servers which should be assigned to the Network Interface.
    #[builder(into)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Option<Vec<String>>,
    /// Does this Network Interface support Accelerated Networking? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enableAcceleratedNetworking")]
    pub r#enable_accelerated_networking: Option<bool>,
    /// Does this Network Interface support IP Forwarding? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enableIpForwarding")]
    pub r#enable_ip_forwarding: Option<bool>,
    /// One or more `ip_configuration` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "ipConfigurations")]
    pub r#ip_configurations: Vec<super::super::types::compute::WindowsVirtualMachineScaleSetNetworkInterfaceIpConfiguration>,
    /// The Name which should be used for this Network Interface. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The ID of a Network Security Group which should be assigned to this Network Interface.
    #[builder(into)]
    #[serde(rename = "networkSecurityGroupId")]
    pub r#network_security_group_id: Option<String>,
    /// Is this the Primary IP Configuration?
    /// 
    /// > **Note:** If multiple `network_interface` blocks are specified, one must be set to `primary`.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WindowsVirtualMachineScaleSetNetworkInterface {
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
                "dns_servers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dns_servers,
                )
                .await,
            );
            map.insert(
                "enable_accelerated_networking".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_accelerated_networking,
                )
                .await,
            );
            map.insert(
                "enable_ip_forwarding".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_ip_forwarding,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WindowsVirtualMachineScaleSetNetworkInterface {
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
                    r#dns_servers: {
                        let field_value = match fields_map.get("dns_servers") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_servers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_accelerated_networking: {
                        let field_value = match fields_map.get("enable_accelerated_networking") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_accelerated_networking' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_ip_forwarding: {
                        let field_value = match fields_map.get("enable_ip_forwarding") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_ip_forwarding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
