#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LaunchTemplateNetworkInterface {
    /// Associate a Carrier IP address with `eth0` for a new network interface. Use this option when you launch an instance in a Wavelength Zone and want to associate a Carrier IP address with the network interface. Boolean value, can be left unset.
    #[builder(into)]
    pub r#associate_carrier_ip_address: Option<String>,
    /// Associate a public ip address with the network interface. Boolean value, can be left unset.
    #[builder(into)]
    pub r#associate_public_ip_address: Option<String>,
    /// Whether the network interface should be destroyed on instance termination.
    #[builder(into)]
    pub r#delete_on_termination: Option<String>,
    /// Description of the network interface.
    #[builder(into)]
    pub r#description: Option<String>,
    /// The integer index of the network interface attachment.
    #[builder(into)]
    pub r#device_index: Option<i32>,
    /// The type of network interface. To create an Elastic Fabric Adapter (EFA), specify `efa`.
    #[builder(into)]
    pub r#interface_type: Option<String>,
    /// The number of secondary private IPv4 addresses to assign to a network interface. Conflicts with `ipv4_addresses`
    #[builder(into)]
    pub r#ipv_4_address_count: Option<i32>,
    /// One or more private IPv4 addresses to associate. Conflicts with `ipv4_address_count`
    #[builder(into)]
    pub r#ipv_4_addresses: Option<Vec<String>>,
    /// The number of IPv4 prefixes to be automatically assigned to the network interface. Conflicts with `ipv4_prefixes`
    #[builder(into)]
    pub r#ipv_4_prefix_count: Option<i32>,
    /// One or more IPv4 prefixes to be assigned to the network interface. Conflicts with `ipv4_prefix_count`
    #[builder(into)]
    pub r#ipv_4_prefixes: Option<Vec<String>>,
    /// The number of IPv6 addresses to assign to a network interface. Conflicts with `ipv6_addresses`
    #[builder(into)]
    pub r#ipv_6_address_count: Option<i32>,
    /// One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet. Conflicts with `ipv6_address_count`
    #[builder(into)]
    pub r#ipv_6_addresses: Option<Vec<String>>,
    /// The number of IPv6 prefixes to be automatically assigned to the network interface. Conflicts with `ipv6_prefixes`
    #[builder(into)]
    pub r#ipv_6_prefix_count: Option<i32>,
    /// One or more IPv6 prefixes to be assigned to the network interface. Conflicts with `ipv6_prefix_count`
    #[builder(into)]
    pub r#ipv_6_prefixes: Option<Vec<String>>,
    /// The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index 0. The default is network card index 0.
    #[builder(into)]
    pub r#network_card_index: Option<i32>,
    /// The ID of the network interface to attach.
    #[builder(into)]
    pub r#network_interface_id: Option<String>,
    /// Whether the first IPv6 GUA will be made the primary IPv6 address.
    #[builder(into)]
    pub r#primary_ipv_6: Option<String>,
    /// The primary private IPv4 address.
    #[builder(into)]
    pub r#private_ip_address: Option<String>,
    /// A list of security group IDs to associate.
    #[builder(into)]
    pub r#security_groups: Option<Vec<String>>,
    /// The VPC Subnet ID to associate.
    #[builder(into)]
    pub r#subnet_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LaunchTemplateNetworkInterface {
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
                    "associateCarrierIpAddress",
                    &self.r#associate_carrier_ip_address,
                ),
                to_pulumi_object_field(
                    "associatePublicIpAddress",
                    &self.r#associate_public_ip_address,
                ),
                to_pulumi_object_field(
                    "deleteOnTermination",
                    &self.r#delete_on_termination,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "deviceIndex",
                    &self.r#device_index,
                ),
                to_pulumi_object_field(
                    "interfaceType",
                    &self.r#interface_type,
                ),
                to_pulumi_object_field(
                    "ipv4AddressCount",
                    &self.r#ipv_4_address_count,
                ),
                to_pulumi_object_field(
                    "ipv4Addresses",
                    &self.r#ipv_4_addresses,
                ),
                to_pulumi_object_field(
                    "ipv4PrefixCount",
                    &self.r#ipv_4_prefix_count,
                ),
                to_pulumi_object_field(
                    "ipv4Prefixes",
                    &self.r#ipv_4_prefixes,
                ),
                to_pulumi_object_field(
                    "ipv6AddressCount",
                    &self.r#ipv_6_address_count,
                ),
                to_pulumi_object_field(
                    "ipv6Addresses",
                    &self.r#ipv_6_addresses,
                ),
                to_pulumi_object_field(
                    "ipv6PrefixCount",
                    &self.r#ipv_6_prefix_count,
                ),
                to_pulumi_object_field(
                    "ipv6Prefixes",
                    &self.r#ipv_6_prefixes,
                ),
                to_pulumi_object_field(
                    "networkCardIndex",
                    &self.r#network_card_index,
                ),
                to_pulumi_object_field(
                    "networkInterfaceId",
                    &self.r#network_interface_id,
                ),
                to_pulumi_object_field(
                    "primaryIpv6",
                    &self.r#primary_ipv_6,
                ),
                to_pulumi_object_field(
                    "privateIpAddress",
                    &self.r#private_ip_address,
                ),
                to_pulumi_object_field(
                    "securityGroups",
                    &self.r#security_groups,
                ),
                to_pulumi_object_field(
                    "subnetId",
                    &self.r#subnet_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("associateCarrierIpAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'associateCarrierIpAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#associate_public_ip_address: {
                        let field_value = match fields_map.get("associatePublicIpAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'associatePublicIpAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delete_on_termination: {
                        let field_value = match fields_map.get("deleteOnTermination") {
                            Some(value) => value,
                            None => bail!("Missing field 'deleteOnTermination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("deviceIndex") {
                            Some(value) => value,
                            None => bail!("Missing field 'deviceIndex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interface_type: {
                        let field_value = match fields_map.get("interfaceType") {
                            Some(value) => value,
                            None => bail!("Missing field 'interfaceType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_4_address_count: {
                        let field_value = match fields_map.get("ipv4AddressCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv4AddressCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_4_addresses: {
                        let field_value = match fields_map.get("ipv4Addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv4Addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_4_prefix_count: {
                        let field_value = match fields_map.get("ipv4PrefixCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv4PrefixCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_4_prefixes: {
                        let field_value = match fields_map.get("ipv4Prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv4Prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_address_count: {
                        let field_value = match fields_map.get("ipv6AddressCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv6AddressCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_addresses: {
                        let field_value = match fields_map.get("ipv6Addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv6Addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_prefix_count: {
                        let field_value = match fields_map.get("ipv6PrefixCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv6PrefixCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_prefixes: {
                        let field_value = match fields_map.get("ipv6Prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv6Prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_card_index: {
                        let field_value = match fields_map.get("networkCardIndex") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkCardIndex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_interface_id: {
                        let field_value = match fields_map.get("networkInterfaceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkInterfaceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary_ipv_6: {
                        let field_value = match fields_map.get("primaryIpv6") {
                            Some(value) => value,
                            None => bail!("Missing field 'primaryIpv6' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip_address: {
                        let field_value = match fields_map.get("privateIpAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateIpAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_groups: {
                        let field_value = match fields_map.get("securityGroups") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityGroups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
