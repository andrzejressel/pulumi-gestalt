#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyIntrusionDetectionTrafficBypass {
    /// The description for this bypass traffic setting.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Specifies a list of destination IP addresses that shall be bypassed by intrusion detection.
    #[builder(into)]
    #[serde(rename = "destinationAddresses")]
    pub r#destination_addresses: Option<Vec<String>>,
    /// Specifies a list of destination IP groups that shall be bypassed by intrusion detection.
    #[builder(into)]
    #[serde(rename = "destinationIpGroups")]
    pub r#destination_ip_groups: Option<Vec<String>>,
    /// Specifies a list of destination IP ports that shall be bypassed by intrusion detection.
    #[builder(into)]
    #[serde(rename = "destinationPorts")]
    pub r#destination_ports: Option<Vec<String>>,
    /// The name which should be used for this bypass traffic setting.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The protocols any of `ANY`, `TCP`, `ICMP`, `UDP` that shall be bypassed by intrusion detection.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// Specifies a list of source addresses that shall be bypassed by intrusion detection.
    #[builder(into)]
    #[serde(rename = "sourceAddresses")]
    pub r#source_addresses: Option<Vec<String>>,
    /// Specifies a list of source IP groups that shall be bypassed by intrusion detection.
    #[builder(into)]
    #[serde(rename = "sourceIpGroups")]
    pub r#source_ip_groups: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirewallPolicyIntrusionDetectionTrafficBypass {
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
                    "destination_ip_groups",
                    &self.r#destination_ip_groups,
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
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "source_addresses",
                    &self.r#source_addresses,
                ),
                to_pulumi_object_field(
                    "source_ip_groups",
                    &self.r#source_ip_groups,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirewallPolicyIntrusionDetectionTrafficBypass {
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
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
