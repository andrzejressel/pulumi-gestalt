#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareClusterNetworkConfig {
    /// Configuration for control plane V2 mode.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "controlPlaneV2Config")]
    pub r#control_plane_v_2_config: Option<Box<super::super::types::gkeonprem::VMwareClusterNetworkConfigControlPlaneV2Config>>,
    /// Configuration settings for a DHCP IP configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dhcpIpConfig")]
    pub r#dhcp_ip_config: Option<Box<super::super::types::gkeonprem::VMwareClusterNetworkConfigDhcpIpConfig>>,
    /// Represents common network settings irrespective of the host's IP address.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "hostConfig")]
    pub r#host_config: Option<Box<super::super::types::gkeonprem::VMwareClusterNetworkConfigHostConfig>>,
    /// All pods in the cluster are assigned an RFC1918 IPv4 address from these ranges.
    /// Only a single range is supported. This field cannot be changed after creation.
    #[builder(into)]
    #[serde(rename = "podAddressCidrBlocks")]
    pub r#pod_address_cidr_blocks: Vec<String>,
    /// All services in the cluster are assigned an RFC1918 IPv4 address
    /// from these ranges. Only a single range is supported.. This field
    /// cannot be changed after creation.
    #[builder(into)]
    #[serde(rename = "serviceAddressCidrBlocks")]
    pub r#service_address_cidr_blocks: Vec<String>,
    /// Configuration settings for a static IP configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "staticIpConfig")]
    pub r#static_ip_config: Option<Box<super::super::types::gkeonprem::VMwareClusterNetworkConfigStaticIpConfig>>,
    /// vcenter_network specifies vCenter network name. Inherited from the admin cluster.
    #[builder(into)]
    #[serde(rename = "vcenterNetwork")]
    pub r#vcenter_network: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VMwareClusterNetworkConfig {
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
                    "control_plane_v_2_config",
                    &self.r#control_plane_v_2_config,
                ),
                to_pulumi_object_field(
                    "dhcp_ip_config",
                    &self.r#dhcp_ip_config,
                ),
                to_pulumi_object_field(
                    "host_config",
                    &self.r#host_config,
                ),
                to_pulumi_object_field(
                    "pod_address_cidr_blocks",
                    &self.r#pod_address_cidr_blocks,
                ),
                to_pulumi_object_field(
                    "service_address_cidr_blocks",
                    &self.r#service_address_cidr_blocks,
                ),
                to_pulumi_object_field(
                    "static_ip_config",
                    &self.r#static_ip_config,
                ),
                to_pulumi_object_field(
                    "vcenter_network",
                    &self.r#vcenter_network,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VMwareClusterNetworkConfig {
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
                    r#control_plane_v_2_config: {
                        let field_value = match fields_map.get("control_plane_v_2_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'control_plane_v_2_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dhcp_ip_config: {
                        let field_value = match fields_map.get("dhcp_ip_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'dhcp_ip_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_config: {
                        let field_value = match fields_map.get("host_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_address_cidr_blocks: {
                        let field_value = match fields_map.get("pod_address_cidr_blocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'pod_address_cidr_blocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_address_cidr_blocks: {
                        let field_value = match fields_map.get("service_address_cidr_blocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_address_cidr_blocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#static_ip_config: {
                        let field_value = match fields_map.get("static_ip_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'static_ip_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vcenter_network: {
                        let field_value = match fields_map.get("vcenter_network") {
                            Some(value) => value,
                            None => bail!("Missing field 'vcenter_network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
