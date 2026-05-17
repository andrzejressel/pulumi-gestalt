#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceFromMachineImageNetworkInterface {
    /// Access configurations, i.e. IPs via which this instance can be accessed via the Internet.
    #[builder(into)]
    #[serde(rename = "accessConfigs")]
    pub r#access_configs: Option<Vec<super::super::types::compute::InstanceFromMachineImageNetworkInterfaceAccessConfig>>,
    /// An array of alias IP ranges for this network interface.
    #[builder(into)]
    #[serde(rename = "aliasIpRanges")]
    pub r#alias_ip_ranges: Option<Vec<super::super::types::compute::InstanceFromMachineImageNetworkInterfaceAliasIpRange>>,
    /// The prefix length of the primary internal IPv6 range.
    #[builder(into)]
    #[serde(rename = "internalIpv6PrefixLength")]
    pub r#internal_ipv_6_prefix_length: Option<i32>,
    /// An array of IPv6 access configurations for this interface. Currently, only one IPv6 access config, DIRECT_IPV6, is supported. If there is no ipv6AccessConfig specified, then this instance will have no external IPv6 Internet access.
    #[builder(into)]
    #[serde(rename = "ipv6AccessConfigs")]
    pub r#ipv_6_access_configs: Option<Vec<super::super::types::compute::InstanceFromMachineImageNetworkInterfaceIpv6AccessConfig>>,
    /// One of EXTERNAL, INTERNAL to indicate whether the IP can be accessed from the Internet. This field is always inherited from its subnetwork.
    #[builder(into)]
    #[serde(rename = "ipv6AccessType")]
    pub r#ipv_6_access_type: Option<String>,
    /// An IPv6 internal network address for this network interface. If not specified, Google Cloud will automatically assign an internal IPv6 address from the instance's subnetwork.
    #[builder(into)]
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: Option<String>,
    /// A unique name for the resource, required by GCE.
    /// Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The name or self_link of the network attached to this interface.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// The URL of the network attachment that this interface should connect to in the following format: projects/{projectNumber}/regions/{region_name}/networkAttachments/{network_attachment_name}.
    #[builder(into)]
    #[serde(rename = "networkAttachment")]
    pub r#network_attachment: Option<String>,
    /// The private IP address assigned to the instance.
    #[builder(into)]
    #[serde(rename = "networkIp")]
    pub r#network_ip: Option<String>,
    /// The type of vNIC to be used on this interface. Possible values:GVNIC, VIRTIO_NET, IDPF, MRDMA, and IRDMA
    #[builder(into)]
    #[serde(rename = "nicType")]
    pub r#nic_type: Option<String>,
    /// The networking queue count that's specified by users for the network interface. Both Rx and Tx queues will be set to this number. It will be empty if not specified.
    #[builder(into)]
    #[serde(rename = "queueCount")]
    pub r#queue_count: Option<i32>,
    /// A full or partial URL to a security policy to add to this instance. If this field is set to an empty string it will remove the associated security policy.
    #[builder(into)]
    #[serde(rename = "securityPolicy")]
    pub r#security_policy: Option<String>,
    /// The stack type for this network interface to identify whether the IPv6 feature is enabled or not. If not specified, IPV4_ONLY will be used.
    #[builder(into)]
    #[serde(rename = "stackType")]
    pub r#stack_type: Option<String>,
    /// The name or self_link of the subnetwork attached to this interface.
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Option<String>,
    /// The project in which the subnetwork belongs.
    #[builder(into)]
    #[serde(rename = "subnetworkProject")]
    pub r#subnetwork_project: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceFromMachineImageNetworkInterface {
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
                    "access_configs",
                    &self.r#access_configs,
                ),
                to_pulumi_object_field(
                    "alias_ip_ranges",
                    &self.r#alias_ip_ranges,
                ),
                to_pulumi_object_field(
                    "internal_ipv_6_prefix_length",
                    &self.r#internal_ipv_6_prefix_length,
                ),
                to_pulumi_object_field(
                    "ipv_6_access_configs",
                    &self.r#ipv_6_access_configs,
                ),
                to_pulumi_object_field(
                    "ipv_6_access_type",
                    &self.r#ipv_6_access_type,
                ),
                to_pulumi_object_field(
                    "ipv_6_address",
                    &self.r#ipv_6_address,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "network",
                    &self.r#network,
                ),
                to_pulumi_object_field(
                    "network_attachment",
                    &self.r#network_attachment,
                ),
                to_pulumi_object_field(
                    "network_ip",
                    &self.r#network_ip,
                ),
                to_pulumi_object_field(
                    "nic_type",
                    &self.r#nic_type,
                ),
                to_pulumi_object_field(
                    "queue_count",
                    &self.r#queue_count,
                ),
                to_pulumi_object_field(
                    "security_policy",
                    &self.r#security_policy,
                ),
                to_pulumi_object_field(
                    "stack_type",
                    &self.r#stack_type,
                ),
                to_pulumi_object_field(
                    "subnetwork",
                    &self.r#subnetwork,
                ),
                to_pulumi_object_field(
                    "subnetwork_project",
                    &self.r#subnetwork_project,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceFromMachineImageNetworkInterface {
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
                    r#network_attachment: {
                        let field_value = match fields_map.get("network_attachment") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_attachment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#security_policy: {
                        let field_value = match fields_map.get("security_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
