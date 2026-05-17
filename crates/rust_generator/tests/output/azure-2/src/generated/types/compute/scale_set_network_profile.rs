#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScaleSetNetworkProfile {
    /// Specifies whether to enable accelerated networking or not.
    #[builder(into)]
    #[serde(rename = "acceleratedNetworking")]
    pub r#accelerated_networking: Option<bool>,
    /// A `dns_settings` block as documented below.
    #[builder(into)]
    #[serde(rename = "dnsSettings")]
    pub r#dns_settings: Option<Box<super::super::types::compute::ScaleSetNetworkProfileDnsSettings>>,
    /// An `ip_configuration` block as documented below.
    #[builder(into)]
    #[serde(rename = "ipConfigurations")]
    pub r#ip_configurations: Vec<super::super::types::compute::ScaleSetNetworkProfileIpConfiguration>,
    /// Whether IP forwarding is enabled on this NIC. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "ipForwarding")]
    pub r#ip_forwarding: Option<bool>,
    /// Specifies the name of the network interface configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies the identifier for the network security group.
    #[builder(into)]
    #[serde(rename = "networkSecurityGroupId")]
    pub r#network_security_group_id: Option<String>,
    /// Indicates whether network interfaces created from the network interface configuration will be the primary NIC of the VM.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScaleSetNetworkProfile {
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
                "accelerated_networking".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#accelerated_networking,
                )
                .await,
            );
            map.insert(
                "dns_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dns_settings,
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
                "ip_forwarding".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_forwarding,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScaleSetNetworkProfile {
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
                    r#accelerated_networking: {
                        let field_value = match fields_map.get("accelerated_networking") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerated_networking' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_settings: {
                        let field_value = match fields_map.get("dns_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#ip_forwarding: {
                        let field_value = match fields_map.get("ip_forwarding") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_forwarding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
