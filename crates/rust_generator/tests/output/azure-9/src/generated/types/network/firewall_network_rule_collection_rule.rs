#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallNetworkRuleCollectionRule {
    /// Specifies a description for the rule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Either a list of destination IP addresses and/or IP ranges, or a list of destination [Service Tags](https://docs.microsoft.com/azure/virtual-network/service-tags-overview#available-service-tags).
    #[builder(into)]
    #[serde(rename = "destinationAddresses")]
    pub r#destination_addresses: Option<Vec<String>>,
    /// A list of destination FQDNS for the rule.
    /// 
    /// > **NOTE** [You must enable DNS Proxy to use FQDNs in your network rules](https://docs.microsoft.com/azure/firewall/fqdn-filtering-network-rules).
    /// 
    /// > **NOTE** At least one of `destination_addresses`, `destination_ip_groups` and `destination_fqdns` must be specified for a rule.
    #[builder(into)]
    #[serde(rename = "destinationFqdns")]
    pub r#destination_fqdns: Option<Vec<String>>,
    /// A list of destination IP Group IDs for the rule.
    #[builder(into)]
    #[serde(rename = "destinationIpGroups")]
    pub r#destination_ip_groups: Option<Vec<String>>,
    /// A list of destination ports.
    #[builder(into)]
    #[serde(rename = "destinationPorts")]
    pub r#destination_ports: Vec<String>,
    /// Specifies the name of the rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A list of protocols. Possible values are `Any`, `ICMP`, `TCP` and `UDP`.
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Vec<String>,
    /// A list of source IP addresses and/or IP ranges.
    #[builder(into)]
    #[serde(rename = "sourceAddresses")]
    pub r#source_addresses: Option<Vec<String>>,
    /// A list of IP Group IDs for the rule.
    /// 
    /// > **NOTE** At least one of `source_addresses` and `source_ip_groups` must be specified for a rule.
    #[builder(into)]
    #[serde(rename = "sourceIpGroups")]
    pub r#source_ip_groups: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirewallNetworkRuleCollectionRule {
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
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "destination_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination_addresses,
                )
                .await,
            );
            map.insert(
                "destination_fqdns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination_fqdns,
                )
                .await,
            );
            map.insert(
                "destination_ip_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination_ip_groups,
                )
                .await,
            );
            map.insert(
                "destination_ports".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination_ports,
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
                "protocols".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protocols,
                )
                .await,
            );
            map.insert(
                "source_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_addresses,
                )
                .await,
            );
            map.insert(
                "source_ip_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_ip_groups,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirewallNetworkRuleCollectionRule {
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_addresses: {
                        let field_value = match fields_map.get("destination_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_fqdns: {
                        let field_value = match fields_map.get("destination_fqdns") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_fqdns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_ip_groups: {
                        let field_value = match fields_map.get("destination_ip_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_ip_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_ports: {
                        let field_value = match fields_map.get("destination_ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#protocols: {
                        let field_value = match fields_map.get("protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_addresses: {
                        let field_value = match fields_map.get("source_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_ip_groups: {
                        let field_value = match fields_map.get("source_ip_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_ip_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
