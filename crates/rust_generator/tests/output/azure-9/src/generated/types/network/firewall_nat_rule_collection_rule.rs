#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallNatRuleCollectionRule {
    /// Specifies a description for the rule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// A list of destination IP addresses and/or IP ranges.
    #[builder(into)]
    #[serde(rename = "destinationAddresses")]
    pub r#destination_addresses: Vec<String>,
    /// A list of destination ports.
    #[builder(into)]
    #[serde(rename = "destinationPorts")]
    pub r#destination_ports: Vec<String>,
    /// Specifies the name of the rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A list of protocols. Possible values are `Any`, `ICMP`, `TCP` and `UDP`. If `action` is `Dnat`, protocols can only be `TCP` and `UDP`.
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Vec<String>,
    /// A list of source IP addresses and/or IP ranges.
    #[builder(into)]
    #[serde(rename = "sourceAddresses")]
    pub r#source_addresses: Option<Vec<String>>,
    /// A list of source IP Group IDs for the rule.
    /// 
    /// > **NOTE** At least one of `source_addresses` and `source_ip_groups` must be specified for a rule.
    #[builder(into)]
    #[serde(rename = "sourceIpGroups")]
    pub r#source_ip_groups: Option<Vec<String>>,
    /// The address of the service behind the Firewall.
    #[builder(into)]
    #[serde(rename = "translatedAddress")]
    pub r#translated_address: String,
    /// The port of the service behind the Firewall.
    #[builder(into)]
    #[serde(rename = "translatedPort")]
    pub r#translated_port: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirewallNatRuleCollectionRule {
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
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "destination_addresses",
                    &self.r#destination_addresses,
                ),
                to_pulumi_object_field(
                    "destination_ports",
                    &self.r#destination_ports,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "protocols",
                    &self.r#protocols,
                ),
                to_pulumi_object_field(
                    "source_addresses",
                    &self.r#source_addresses,
                ),
                to_pulumi_object_field(
                    "source_ip_groups",
                    &self.r#source_ip_groups,
                ),
                to_pulumi_object_field(
                    "translated_address",
                    &self.r#translated_address,
                ),
                to_pulumi_object_field(
                    "translated_port",
                    &self.r#translated_port,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirewallNatRuleCollectionRule {
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
                    r#translated_address: {
                        let field_value = match fields_map.get("translated_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'translated_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#translated_port: {
                        let field_value = match fields_map.get("translated_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'translated_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
