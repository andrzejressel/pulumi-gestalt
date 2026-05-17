#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LaunchTemplateNetworkInterface {
    /// Associate a Carrier IP address with `eth0` for a new network interface. Use this option when you launch an instance in a Wavelength Zone and want to associate a Carrier IP address with the network interface. Boolean value, can be left unset.
    #[builder(into)]
    #[serde(rename = "associateCarrierIpAddress")]
    pub r#associate_carrier_ip_address: Option<String>,
    /// Associate a public ip address with the network interface. Boolean value, can be left unset.
    #[builder(into)]
    #[serde(rename = "associatePublicIpAddress")]
    pub r#associate_public_ip_address: Option<String>,
    /// Whether the network interface should be destroyed on instance termination.
    #[builder(into)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Option<String>,
    /// Description of the network interface.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The integer index of the network interface attachment.
    #[builder(into)]
    #[serde(rename = "deviceIndex")]
    pub r#device_index: Option<i32>,
    /// The type of network interface. To create an Elastic Fabric Adapter (EFA), specify `efa`.
    #[builder(into)]
    #[serde(rename = "interfaceType")]
    pub r#interface_type: Option<String>,
    /// The number of secondary private IPv4 addresses to assign to a network interface. Conflicts with `ipv4_addresses`
    #[builder(into)]
    #[serde(rename = "ipv4AddressCount")]
    pub r#ipv_4_address_count: Option<i32>,
    /// One or more private IPv4 addresses to associate. Conflicts with `ipv4_address_count`
    #[builder(into)]
    #[serde(rename = "ipv4Addresses")]
    pub r#ipv_4_addresses: Option<Vec<String>>,
    /// The number of IPv4 prefixes to be automatically assigned to the network interface. Conflicts with `ipv4_prefixes`
    #[builder(into)]
    #[serde(rename = "ipv4PrefixCount")]
    pub r#ipv_4_prefix_count: Option<i32>,
    /// One or more IPv4 prefixes to be assigned to the network interface. Conflicts with `ipv4_prefix_count`
    #[builder(into)]
    #[serde(rename = "ipv4Prefixes")]
    pub r#ipv_4_prefixes: Option<Vec<String>>,
    /// The number of IPv6 addresses to assign to a network interface. Conflicts with `ipv6_addresses`
    #[builder(into)]
    #[serde(rename = "ipv6AddressCount")]
    pub r#ipv_6_address_count: Option<i32>,
    /// One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet. Conflicts with `ipv6_address_count`
    #[builder(into)]
    #[serde(rename = "ipv6Addresses")]
    pub r#ipv_6_addresses: Option<Vec<String>>,
    /// The number of IPv6 prefixes to be automatically assigned to the network interface. Conflicts with `ipv6_prefixes`
    #[builder(into)]
    #[serde(rename = "ipv6PrefixCount")]
    pub r#ipv_6_prefix_count: Option<i32>,
    /// One or more IPv6 prefixes to be assigned to the network interface. Conflicts with `ipv6_prefix_count`
    #[builder(into)]
    #[serde(rename = "ipv6Prefixes")]
    pub r#ipv_6_prefixes: Option<Vec<String>>,
    /// The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index 0. The default is network card index 0.
    #[builder(into)]
    #[serde(rename = "networkCardIndex")]
    pub r#network_card_index: Option<i32>,
    /// The ID of the network interface to attach.
    #[builder(into)]
    #[serde(rename = "networkInterfaceId")]
    pub r#network_interface_id: Option<String>,
    /// Whether the first IPv6 GUA will be made the primary IPv6 address.
    #[builder(into)]
    #[serde(rename = "primaryIpv6")]
    pub r#primary_ipv_6: Option<String>,
    /// The primary private IPv4 address.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Option<String>,
    /// A list of security group IDs to associate.
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Option<Vec<String>>,
    /// The VPC Subnet ID to associate.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LaunchTemplateNetworkInterface {
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
                "associate_carrier_ip_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#associate_carrier_ip_address,
                )
                .await,
            );
            map.insert(
                "associate_public_ip_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#associate_public_ip_address,
                )
                .await,
            );
            map.insert(
                "delete_on_termination".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#delete_on_termination,
                )
                .await,
            );
            map.insert(
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "device_index".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_index,
                )
                .await,
            );
            map.insert(
                "interface_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#interface_type,
                )
                .await,
            );
            map.insert(
                "ipv_4_address_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_4_address_count,
                )
                .await,
            );
            map.insert(
                "ipv_4_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_4_addresses,
                )
                .await,
            );
            map.insert(
                "ipv_4_prefix_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_4_prefix_count,
                )
                .await,
            );
            map.insert(
                "ipv_4_prefixes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_4_prefixes,
                )
                .await,
            );
            map.insert(
                "ipv_6_address_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_6_address_count,
                )
                .await,
            );
            map.insert(
                "ipv_6_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_6_addresses,
                )
                .await,
            );
            map.insert(
                "ipv_6_prefix_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_6_prefix_count,
                )
                .await,
            );
            map.insert(
                "ipv_6_prefixes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_6_prefixes,
                )
                .await,
            );
            map.insert(
                "network_card_index".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_card_index,
                )
                .await,
            );
            map.insert(
                "network_interface_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_interface_id,
                )
                .await,
            );
            map.insert(
                "primary_ipv_6".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#primary_ipv_6,
                )
                .await,
            );
            map.insert(
                "private_ip_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_ip_address,
                )
                .await,
            );
            map.insert(
                "security_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_groups,
                )
                .await,
            );
            map.insert(
                "subnet_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnet_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LaunchTemplateNetworkInterface {
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
