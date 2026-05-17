#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionInstanceTemplateNetworkInterface {
    #[builder(into)]
    #[serde(rename = "accessConfigs")]
    pub r#access_configs: Option<Vec<super::super::types::compute::RegionInstanceTemplateNetworkInterfaceAccessConfig>>,
    /// An
    /// array of alias IP ranges for this network interface. Can only be specified for network
    /// interfaces on subnet-mode networks. Structure documented below.
    #[builder(into)]
    #[serde(rename = "aliasIpRanges")]
    pub r#alias_ip_ranges: Option<Vec<super::super::types::compute::RegionInstanceTemplateNetworkInterfaceAliasIpRange>>,
    /// The prefix length of the primary internal IPv6 range.
    #[builder(into)]
    #[serde(rename = "internalIpv6PrefixLength")]
    pub r#internal_ipv_6_prefix_length: Option<i32>,
    /// An array of IPv6 access configurations for this interface.
    /// Currently, only one IPv6 access config, DIRECT_IPV6, is supported. If there is no ipv6AccessConfig
    /// specified, then this instance will have no external IPv6 Internet access. Structure documented below.
    #[builder(into)]
    #[serde(rename = "ipv6AccessConfigs")]
    pub r#ipv_6_access_configs: Option<Vec<super::super::types::compute::RegionInstanceTemplateNetworkInterfaceIpv6AccessConfig>>,
    /// One of EXTERNAL, INTERNAL to indicate whether the IP can be accessed from the Internet. This field is always inherited from its subnetwork.
    #[builder(into)]
    #[serde(rename = "ipv6AccessType")]
    pub r#ipv_6_access_type: Option<String>,
    /// An IPv6 internal network address for this network interface. If not specified, Google Cloud will automatically assign an internal IPv6 address from the instance's subnetwork.
    #[builder(into)]
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: Option<String>,
    /// The name of the network_interface.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The name or self_link of the network to attach this interface to.
    /// Use `network` attribute for Legacy or Auto subnetted networks and
    /// `subnetwork` for custom subnetted networks.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// The private IP address to assign to the instance. If
    /// empty, the address will be automatically assigned.
    #[builder(into)]
    #[serde(rename = "networkIp")]
    pub r#network_ip: Option<String>,
    /// The type of vNIC to be used on this interface. Possible values: GVNIC, VIRTIO_NET. In the beta provider the additional values of MRDMA and IRDMA are supported.
    #[builder(into)]
    #[serde(rename = "nicType")]
    pub r#nic_type: Option<String>,
    /// The networking queue count that's specified by users for the network interface. Both Rx and Tx queues will be set to this number. It will be empty if not specified.
    #[builder(into)]
    #[serde(rename = "queueCount")]
    pub r#queue_count: Option<i32>,
    /// The stack type for this network interface to identify whether the IPv6 feature is enabled or not. Values are IPV4_IPV6 or IPV4_ONLY. If not specified, IPV4_ONLY will be used.
    #[builder(into)]
    #[serde(rename = "stackType")]
    pub r#stack_type: Option<String>,
    /// the name of the subnetwork to attach this interface
    /// to. The subnetwork must exist in the same `region` this instance will be
    /// created in. Either `network` or `subnetwork` must be provided.
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Option<String>,
    /// The ID of the project in which the subnetwork belongs.
    /// If it is not provided, the provider project is used.
    #[builder(into)]
    #[serde(rename = "subnetworkProject")]
    pub r#subnetwork_project: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionInstanceTemplateNetworkInterface {
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
                "access_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#access_configs,
                )
                .await,
            );
            map.insert(
                "alias_ip_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#alias_ip_ranges,
                )
                .await,
            );
            map.insert(
                "internal_ipv_6_prefix_length".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#internal_ipv_6_prefix_length,
                )
                .await,
            );
            map.insert(
                "ipv_6_access_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_6_access_configs,
                )
                .await,
            );
            map.insert(
                "ipv_6_access_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_6_access_type,
                )
                .await,
            );
            map.insert(
                "ipv_6_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_6_address,
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
                "network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network,
                )
                .await,
            );
            map.insert(
                "network_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_ip,
                )
                .await,
            );
            map.insert(
                "nic_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nic_type,
                )
                .await,
            );
            map.insert(
                "queue_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#queue_count,
                )
                .await,
            );
            map.insert(
                "stack_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#stack_type,
                )
                .await,
            );
            map.insert(
                "subnetwork".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnetwork,
                )
                .await,
            );
            map.insert(
                "subnetwork_project".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnetwork_project,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionInstanceTemplateNetworkInterface {
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
                    r#access_configs: {
                        let field_value = match fields_map.get("access_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#alias_ip_ranges: {
                        let field_value = match fields_map.get("alias_ip_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'alias_ip_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#internal_ipv_6_prefix_length: {
                        let field_value = match fields_map.get("internal_ipv_6_prefix_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'internal_ipv_6_prefix_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_access_configs: {
                        let field_value = match fields_map.get("ipv_6_access_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6_access_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_access_type: {
                        let field_value = match fields_map.get("ipv_6_access_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6_access_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_address: {
                        let field_value = match fields_map.get("ipv_6_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#network: {
                        let field_value = match fields_map.get("network") {
                            Some(value) => value,
                            None => bail!("Missing field 'network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_ip: {
                        let field_value = match fields_map.get("network_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nic_type: {
                        let field_value = match fields_map.get("nic_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'nic_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#queue_count: {
                        let field_value = match fields_map.get("queue_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'queue_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stack_type: {
                        let field_value = match fields_map.get("stack_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'stack_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnetwork: {
                        let field_value = match fields_map.get("subnetwork") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetwork' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnetwork_project: {
                        let field_value = match fields_map.get("subnetwork_project") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetwork_project' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
