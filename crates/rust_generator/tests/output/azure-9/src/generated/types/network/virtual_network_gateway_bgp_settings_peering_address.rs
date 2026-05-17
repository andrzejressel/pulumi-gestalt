#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNetworkGatewayBgpSettingsPeeringAddress {
    /// A list of Azure custom APIPA addresses assigned to the BGP peer of the Virtual Network Gateway.
    /// 
    /// > **Note:** The valid range for the reserved APIPA address in Azure Public is from `169.254.21.0` to `169.254.22.255`.
    #[builder(into)]
    #[serde(rename = "apipaAddresses")]
    pub r#apipa_addresses: Option<Vec<String>>,
    /// A list of peering address assigned to the BGP peer of the Virtual Network Gateway.
    #[builder(into)]
    #[serde(rename = "defaultAddresses")]
    pub r#default_addresses: Option<Vec<String>>,
    /// The name of the IP configuration of this Virtual Network Gateway. In case there are multiple `ip_configuration` blocks defined, this property is **required** to specify.
    #[builder(into)]
    #[serde(rename = "ipConfigurationName")]
    pub r#ip_configuration_name: Option<String>,
    /// A list of tunnel IP addresses assigned to the BGP peer of the Virtual Network Gateway.
    #[builder(into)]
    #[serde(rename = "tunnelIpAddresses")]
    pub r#tunnel_ip_addresses: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualNetworkGatewayBgpSettingsPeeringAddress {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "apipa_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#apipa_addresses,
                )
                .await,
            );
            map.insert(
                "default_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_addresses,
                )
                .await,
            );
            map.insert(
                "ip_configuration_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_configuration_name,
                )
                .await,
            );
            map.insert(
                "tunnel_ip_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tunnel_ip_addresses,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualNetworkGatewayBgpSettingsPeeringAddress {
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
                    r#apipa_addresses: {
                        let field_value = match fields_map.get("apipa_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'apipa_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_addresses: {
                        let field_value = match fields_map.get("default_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_configuration_name: {
                        let field_value = match fields_map.get("ip_configuration_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_configuration_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tunnel_ip_addresses: {
                        let field_value = match fields_map.get("tunnel_ip_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'tunnel_ip_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
