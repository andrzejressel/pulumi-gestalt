#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceNetworkInterface {
    /// Access configurations, i.e. IPs via which this instance can be accessed via the Internet.
    #[builder(into)]
    pub r#access_configs: Option<Vec<super::super::types::compute::InstanceNetworkInterfaceAccessConfig>>,
    /// An
    /// array of alias IP ranges for this network interface. Can only be specified for network
    /// interfaces on subnet-mode networks. Structure documented below.
    #[builder(into)]
    pub r#alias_ip_ranges: Option<Vec<super::super::types::compute::InstanceNetworkInterfaceAliasIpRange>>,
    /// The prefix length of the primary internal IPv6 range.
    #[builder(into)]
    pub r#internal_ipv_6_prefix_length: Option<i32>,
    /// An array of IPv6 access configurations for this interface.
    /// Currently, only one IPv6 access config, DIRECT_IPV6, is supported. If there is no ipv6AccessConfig
    /// specified, then this instance will have no external IPv6 Internet access. Structure documented below.
    #[builder(into)]
    pub r#ipv_6_access_configs: Option<Vec<super::super::types::compute::InstanceNetworkInterfaceIpv6AccessConfig>>,
    /// One of EXTERNAL, INTERNAL to indicate whether the IP can be accessed from the Internet.
    /// This field is always inherited from its subnetwork.
    #[builder(into)]
    pub r#ipv_6_access_type: Option<String>,
    /// An IPv6 internal network address for this network interface. If not specified, Google Cloud will automatically assign an internal IPv6 address from the instance's subnetwork.
    #[builder(into)]
    pub r#ipv_6_address: Option<String>,
    /// A unique name for the resource, required by GCE.
    /// Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#name: Option<String>,
    /// The name or self_link of the network to attach this interface to.
    /// Either `network` or `subnetwork` must be provided. If network isn't provided it will
    /// be inferred from the subnetwork.
    #[builder(into)]
    pub r#network: Option<String>,
    /// The URL of the network attachment that this interface should connect to in the following format: `projects/{projectNumber}/regions/{region_name}/networkAttachments/{network_attachment_name}`.
    #[builder(into)]
    pub r#network_attachment: Option<String>,
    /// The private IP address to assign to the instance. If
    /// empty, the address will be automatically assigned.
    #[builder(into)]
    pub r#network_ip: Option<String>,
    /// The type of vNIC to be used on this interface. Possible values: GVNIC, VIRTIO_NET, IDPF. In the beta provider the additional values of MRDMA and IRDMA are supported.
    #[builder(into)]
    pub r#nic_type: Option<String>,
    /// The networking queue count that's specified by users for the network interface. Both Rx and Tx queues will be set to this number. It will be empty if not specified.
    #[builder(into)]
    pub r#queue_count: Option<i32>,
    /// A full or partial URL to a security policy to add to this instance. If this field is set to an empty string it will remove the associated security policy.
    #[builder(into)]
    pub r#security_policy: Option<String>,
    /// The stack type for this network interface to identify whether the IPv6 feature is enabled or not. Values are IPV4_IPV6 or IPV4_ONLY. If not specified, IPV4_ONLY will be used.
    #[builder(into)]
    pub r#stack_type: Option<String>,
    /// The name or self_link of the subnetwork to attach this
    /// interface to. Either `network` or `subnetwork` must be provided. If network isn't provided
    /// it will be inferred from the subnetwork. The subnetwork must exist in the same region this
    /// instance will be created in. If the network resource is in
    /// [legacy](https://cloud.google.com/vpc/docs/legacy) mode, do not specify this field. If the
    /// network is in auto subnet mode, specifying the subnetwork is optional. If the network is
    /// in custom subnet mode, specifying the subnetwork is required.
    #[builder(into)]
    pub r#subnetwork: Option<String>,
    /// The project in which the subnetwork belongs.
    /// If the `subnetwork` is a self_link, this field is set to the project
    /// defined in the subnetwork self_link. If the `subnetwork` is a name and this
    /// field is not provided, the provider project is used.
    #[builder(into)]
    pub r#subnetwork_project: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceNetworkInterface {
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
                    "accessConfigs",
                    &self.r#access_configs,
                ),
                to_pulumi_object_field(
                    "aliasIpRanges",
                    &self.r#alias_ip_ranges,
                ),
                to_pulumi_object_field(
                    "internalIpv6PrefixLength",
                    &self.r#internal_ipv_6_prefix_length,
                ),
                to_pulumi_object_field(
                    "ipv6AccessConfigs",
                    &self.r#ipv_6_access_configs,
                ),
                to_pulumi_object_field(
                    "ipv6AccessType",
                    &self.r#ipv_6_access_type,
                ),
                to_pulumi_object_field(
                    "ipv6Address",
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
                    "networkAttachment",
                    &self.r#network_attachment,
                ),
                to_pulumi_object_field(
                    "networkIp",
                    &self.r#network_ip,
                ),
                to_pulumi_object_field(
                    "nicType",
                    &self.r#nic_type,
                ),
                to_pulumi_object_field(
                    "queueCount",
                    &self.r#queue_count,
                ),
                to_pulumi_object_field(
                    "securityPolicy",
                    &self.r#security_policy,
                ),
                to_pulumi_object_field(
                    "stackType",
                    &self.r#stack_type,
                ),
                to_pulumi_object_field(
                    "subnetwork",
                    &self.r#subnetwork,
                ),
                to_pulumi_object_field(
                    "subnetworkProject",
                    &self.r#subnetwork_project,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceNetworkInterface {
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
                        let field_value = match fields_map.get("accessConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'accessConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#alias_ip_ranges: {
                        let field_value = match fields_map.get("aliasIpRanges") {
                            Some(value) => value,
                            None => bail!("Missing field 'aliasIpRanges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#internal_ipv_6_prefix_length: {
                        let field_value = match fields_map.get("internalIpv6PrefixLength") {
                            Some(value) => value,
                            None => bail!("Missing field 'internalIpv6PrefixLength' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_access_configs: {
                        let field_value = match fields_map.get("ipv6AccessConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv6AccessConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_access_type: {
                        let field_value = match fields_map.get("ipv6AccessType") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv6AccessType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_address: {
                        let field_value = match fields_map.get("ipv6Address") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv6Address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("networkAttachment") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkAttachment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_ip: {
                        let field_value = match fields_map.get("networkIp") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkIp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nic_type: {
                        let field_value = match fields_map.get("nicType") {
                            Some(value) => value,
                            None => bail!("Missing field 'nicType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#queue_count: {
                        let field_value = match fields_map.get("queueCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'queueCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_policy: {
                        let field_value = match fields_map.get("securityPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stack_type: {
                        let field_value = match fields_map.get("stackType") {
                            Some(value) => value,
                            None => bail!("Missing field 'stackType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("subnetworkProject") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetworkProject' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
