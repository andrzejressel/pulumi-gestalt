#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLaunchTemplateNetworkInterface {
    #[builder(into)]
    #[serde(rename = "associateCarrierIpAddress")]
    pub r#associate_carrier_ip_address: String,
    #[builder(into)]
    #[serde(rename = "associatePublicIpAddress")]
    pub r#associate_public_ip_address: Option<bool>,
    #[builder(into)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Option<bool>,
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    #[builder(into)]
    #[serde(rename = "deviceIndex")]
    pub r#device_index: i32,
    #[builder(into)]
    #[serde(rename = "interfaceType")]
    pub r#interface_type: String,
    #[builder(into)]
    #[serde(rename = "ipv4AddressCount")]
    pub r#ipv_4_address_count: i32,
    #[builder(into)]
    #[serde(rename = "ipv4Addresses")]
    pub r#ipv_4_addresses: Vec<String>,
    #[builder(into)]
    #[serde(rename = "ipv4PrefixCount")]
    pub r#ipv_4_prefix_count: i32,
    #[builder(into)]
    #[serde(rename = "ipv4Prefixes")]
    pub r#ipv_4_prefixes: Vec<String>,
    #[builder(into)]
    #[serde(rename = "ipv6AddressCount")]
    pub r#ipv_6_address_count: i32,
    #[builder(into)]
    #[serde(rename = "ipv6Addresses")]
    pub r#ipv_6_addresses: Vec<String>,
    #[builder(into)]
    #[serde(rename = "ipv6PrefixCount")]
    pub r#ipv_6_prefix_count: i32,
    #[builder(into)]
    #[serde(rename = "ipv6Prefixes")]
    pub r#ipv_6_prefixes: Vec<String>,
    #[builder(into)]
    #[serde(rename = "networkCardIndex")]
    pub r#network_card_index: i32,
    #[builder(into)]
    #[serde(rename = "networkInterfaceId")]
    pub r#network_interface_id: String,
    #[builder(into)]
    #[serde(rename = "primaryIpv6")]
    pub r#primary_ipv_6: String,
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: String,
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Vec<String>,
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLaunchTemplateNetworkInterface {
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
                    "associate_carrier_ip_address",
                    &self.r#associate_carrier_ip_address,
                ),
                to_pulumi_object_field(
                    "associate_public_ip_address",
                    &self.r#associate_public_ip_address,
                ),
                to_pulumi_object_field(
                    "delete_on_termination",
                    &self.r#delete_on_termination,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "device_index",
                    &self.r#device_index,
                ),
                to_pulumi_object_field(
                    "interface_type",
                    &self.r#interface_type,
                ),
                to_pulumi_object_field(
                    "ipv_4_address_count",
                    &self.r#ipv_4_address_count,
                ),
                to_pulumi_object_field(
                    "ipv_4_addresses",
                    &self.r#ipv_4_addresses,
                ),
                to_pulumi_object_field(
                    "ipv_4_prefix_count",
                    &self.r#ipv_4_prefix_count,
                ),
                to_pulumi_object_field(
                    "ipv_4_prefixes",
                    &self.r#ipv_4_prefixes,
                ),
                to_pulumi_object_field(
                    "ipv_6_address_count",
                    &self.r#ipv_6_address_count,
                ),
                to_pulumi_object_field(
                    "ipv_6_addresses",
                    &self.r#ipv_6_addresses,
                ),
                to_pulumi_object_field(
                    "ipv_6_prefix_count",
                    &self.r#ipv_6_prefix_count,
                ),
                to_pulumi_object_field(
                    "ipv_6_prefixes",
                    &self.r#ipv_6_prefixes,
                ),
                to_pulumi_object_field(
                    "network_card_index",
                    &self.r#network_card_index,
                ),
                to_pulumi_object_field(
                    "network_interface_id",
                    &self.r#network_interface_id,
                ),
                to_pulumi_object_field(
                    "primary_ipv_6",
                    &self.r#primary_ipv_6,
                ),
                to_pulumi_object_field(
                    "private_ip_address",
                    &self.r#private_ip_address,
                ),
                to_pulumi_object_field(
                    "security_groups",
                    &self.r#security_groups,
                ),
                to_pulumi_object_field(
                    "subnet_id",
                    &self.r#subnet_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLaunchTemplateNetworkInterface {
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
                    r#associate_carrier_ip_address: {
                        let field_value = match fields_map.get("associate_carrier_ip_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'associate_carrier_ip_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#associate_public_ip_address: {
                        let field_value = match fields_map.get("associate_public_ip_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'associate_public_ip_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delete_on_termination: {
                        let field_value = match fields_map.get("delete_on_termination") {
                            Some(value) => value,
                            None => bail!("Missing field 'delete_on_termination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#device_index: {
                        let field_value = match fields_map.get("device_index") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_index' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interface_type: {
                        let field_value = match fields_map.get("interface_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'interface_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_4_address_count: {
                        let field_value = match fields_map.get("ipv_4_address_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_4_address_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_4_addresses: {
                        let field_value = match fields_map.get("ipv_4_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_4_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_4_prefix_count: {
                        let field_value = match fields_map.get("ipv_4_prefix_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_4_prefix_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_4_prefixes: {
                        let field_value = match fields_map.get("ipv_4_prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_4_prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_address_count: {
                        let field_value = match fields_map.get("ipv_6_address_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6_address_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_addresses: {
                        let field_value = match fields_map.get("ipv_6_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_prefix_count: {
                        let field_value = match fields_map.get("ipv_6_prefix_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6_prefix_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_prefixes: {
                        let field_value = match fields_map.get("ipv_6_prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6_prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_card_index: {
                        let field_value = match fields_map.get("network_card_index") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_card_index' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_interface_id: {
                        let field_value = match fields_map.get("network_interface_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_interface_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary_ipv_6: {
                        let field_value = match fields_map.get("primary_ipv_6") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary_ipv_6' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip_address: {
                        let field_value = match fields_map.get("private_ip_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_ip_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_groups: {
                        let field_value = match fields_map.get("security_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
