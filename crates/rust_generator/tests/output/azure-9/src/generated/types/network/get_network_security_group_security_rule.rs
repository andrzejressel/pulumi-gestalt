#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkSecurityGroupSecurityRule {
    /// Is network traffic is allowed or denied?
    #[builder(into)]
    #[serde(rename = "access")]
    pub r#access: String,
    /// The description for this rule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// CIDR or destination IP range or * to match any IP.
    #[builder(into)]
    #[serde(rename = "destinationAddressPrefix")]
    pub r#destination_address_prefix: String,
    /// A list of CIDRs or destination IP ranges.
    #[builder(into)]
    #[serde(rename = "destinationAddressPrefixes")]
    pub r#destination_address_prefixes: Vec<String>,
    /// A List of destination Application Security Group IDs
    #[builder(into)]
    #[serde(rename = "destinationApplicationSecurityGroupIds")]
    pub r#destination_application_security_group_ids: Option<Vec<String>>,
    /// The Destination Port or Range.
    #[builder(into)]
    #[serde(rename = "destinationPortRange")]
    pub r#destination_port_range: String,
    #[builder(into)]
    #[serde(rename = "destinationPortRanges")]
    pub r#destination_port_ranges: Vec<String>,
    /// The direction specifies if rule will be evaluated on incoming or outgoing traffic.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: String,
    /// Specifies the Name of the Network Security Group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The priority of the rule
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// The network protocol this rule applies to.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// CIDR or source IP range or * to match any IP.
    #[builder(into)]
    #[serde(rename = "sourceAddressPrefix")]
    pub r#source_address_prefix: String,
    /// A list of CIDRs or source IP ranges.
    #[builder(into)]
    #[serde(rename = "sourceAddressPrefixes")]
    pub r#source_address_prefixes: Vec<String>,
    /// A List of source Application Security Group IDs
    #[builder(into)]
    #[serde(rename = "sourceApplicationSecurityGroupIds")]
    pub r#source_application_security_group_ids: Option<Vec<String>>,
    /// The Source Port or Range.
    #[builder(into)]
    #[serde(rename = "sourcePortRange")]
    pub r#source_port_range: String,
    #[builder(into)]
    #[serde(rename = "sourcePortRanges")]
    pub r#source_port_ranges: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNetworkSecurityGroupSecurityRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "access",
                    &self.r#access,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "destination_address_prefix",
                    &self.r#destination_address_prefix,
                ),
                to_pulumi_object_field(
                    "destination_address_prefixes",
                    &self.r#destination_address_prefixes,
                ),
                to_pulumi_object_field(
                    "destination_application_security_group_ids",
                    &self.r#destination_application_security_group_ids,
                ),
                to_pulumi_object_field(
                    "destination_port_range",
                    &self.r#destination_port_range,
                ),
                to_pulumi_object_field(
                    "destination_port_ranges",
                    &self.r#destination_port_ranges,
                ),
                to_pulumi_object_field(
                    "direction",
                    &self.r#direction,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "priority",
                    &self.r#priority,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "source_address_prefix",
                    &self.r#source_address_prefix,
                ),
                to_pulumi_object_field(
                    "source_address_prefixes",
                    &self.r#source_address_prefixes,
                ),
                to_pulumi_object_field(
                    "source_application_security_group_ids",
                    &self.r#source_application_security_group_ids,
                ),
                to_pulumi_object_field(
                    "source_port_range",
                    &self.r#source_port_range,
                ),
                to_pulumi_object_field(
                    "source_port_ranges",
                    &self.r#source_port_ranges,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetNetworkSecurityGroupSecurityRule {
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
                    r#access: {
                        let field_value = match fields_map.get("access") {
                            Some(value) => value,
                            None => bail!("Missing field 'access' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_address_prefix: {
                        let field_value = match fields_map.get("destination_address_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_address_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_address_prefixes: {
                        let field_value = match fields_map.get("destination_address_prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_address_prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_application_security_group_ids: {
                        let field_value = match fields_map.get("destination_application_security_group_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_application_security_group_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_port_range: {
                        let field_value = match fields_map.get("destination_port_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_port_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_port_ranges: {
                        let field_value = match fields_map.get("destination_port_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_port_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#direction: {
                        let field_value = match fields_map.get("direction") {
                            Some(value) => value,
                            None => bail!("Missing field 'direction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#priority: {
                        let field_value = match fields_map.get("priority") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#source_address_prefix: {
                        let field_value = match fields_map.get("source_address_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_address_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_address_prefixes: {
                        let field_value = match fields_map.get("source_address_prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_address_prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_application_security_group_ids: {
                        let field_value = match fields_map.get("source_application_security_group_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_application_security_group_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_port_range: {
                        let field_value = match fields_map.get("source_port_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_port_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_port_ranges: {
                        let field_value = match fields_map.get("source_port_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_port_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
