#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkSecurityGroupSecurityRule {
    /// Is network traffic is allowed or denied?
    #[builder(into)]
    pub r#access: String,
    /// The description for this rule.
    #[builder(into)]
    pub r#description: String,
    /// CIDR or destination IP range or * to match any IP.
    #[builder(into)]
    pub r#destination_address_prefix: String,
    /// A list of CIDRs or destination IP ranges.
    #[builder(into)]
    pub r#destination_address_prefixes: Vec<String>,
    /// A List of destination Application Security Group IDs
    #[builder(into)]
    pub r#destination_application_security_group_ids: Option<Vec<String>>,
    /// The Destination Port or Range.
    #[builder(into)]
    pub r#destination_port_range: String,
    #[builder(into)]
    pub r#destination_port_ranges: Vec<String>,
    /// The direction specifies if rule will be evaluated on incoming or outgoing traffic.
    #[builder(into)]
    pub r#direction: String,
    /// Specifies the Name of the Network Security Group.
    #[builder(into)]
    pub r#name: String,
    /// The priority of the rule
    #[builder(into)]
    pub r#priority: i32,
    /// The network protocol this rule applies to.
    #[builder(into)]
    pub r#protocol: String,
    /// CIDR or source IP range or * to match any IP.
    #[builder(into)]
    pub r#source_address_prefix: String,
    /// A list of CIDRs or source IP ranges.
    #[builder(into)]
    pub r#source_address_prefixes: Vec<String>,
    /// A List of source Application Security Group IDs
    #[builder(into)]
    pub r#source_application_security_group_ids: Option<Vec<String>>,
    /// The Source Port or Range.
    #[builder(into)]
    pub r#source_port_range: String,
    #[builder(into)]
    pub r#source_port_ranges: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNetworkSecurityGroupSecurityRule {
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
                    "access",
                    &self.r#access,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "destinationAddressPrefix",
                    &self.r#destination_address_prefix,
                ),
                to_pulumi_object_field(
                    "destinationAddressPrefixes",
                    &self.r#destination_address_prefixes,
                ),
                to_pulumi_object_field(
                    "destinationApplicationSecurityGroupIds",
                    &self.r#destination_application_security_group_ids,
                ),
                to_pulumi_object_field(
                    "destinationPortRange",
                    &self.r#destination_port_range,
                ),
                to_pulumi_object_field(
                    "destinationPortRanges",
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
                    "sourceAddressPrefix",
                    &self.r#source_address_prefix,
                ),
                to_pulumi_object_field(
                    "sourceAddressPrefixes",
                    &self.r#source_address_prefixes,
                ),
                to_pulumi_object_field(
                    "sourceApplicationSecurityGroupIds",
                    &self.r#source_application_security_group_ids,
                ),
                to_pulumi_object_field(
                    "sourcePortRange",
                    &self.r#source_port_range,
                ),
                to_pulumi_object_field(
                    "sourcePortRanges",
                    &self.r#source_port_ranges,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("destinationAddressPrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationAddressPrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_address_prefixes: {
                        let field_value = match fields_map.get("destinationAddressPrefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationAddressPrefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_application_security_group_ids: {
                        let field_value = match fields_map.get("destinationApplicationSecurityGroupIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationApplicationSecurityGroupIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_port_range: {
                        let field_value = match fields_map.get("destinationPortRange") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationPortRange' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_port_ranges: {
                        let field_value = match fields_map.get("destinationPortRanges") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationPortRanges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("sourceAddressPrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceAddressPrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_address_prefixes: {
                        let field_value = match fields_map.get("sourceAddressPrefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceAddressPrefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_application_security_group_ids: {
                        let field_value = match fields_map.get("sourceApplicationSecurityGroupIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceApplicationSecurityGroupIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_port_range: {
                        let field_value = match fields_map.get("sourcePortRange") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourcePortRange' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_port_ranges: {
                        let field_value = match fields_map.get("sourcePortRanges") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourcePortRanges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
