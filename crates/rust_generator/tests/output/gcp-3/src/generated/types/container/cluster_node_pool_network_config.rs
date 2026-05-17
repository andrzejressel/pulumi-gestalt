#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNodePoolNetworkConfig {
    /// We specify the additional node networks for this node pool using this list. Each node network corresponds to an additional interface
    #[builder(into)]
    #[serde(rename = "additionalNodeNetworkConfigs")]
    pub r#additional_node_network_configs: Option<Vec<super::super::types::container::ClusterNodePoolNetworkConfigAdditionalNodeNetworkConfig>>,
    /// We specify the additional pod networks for this node pool using this list. Each pod network corresponds to an additional alias IP range for the node
    #[builder(into)]
    #[serde(rename = "additionalPodNetworkConfigs")]
    pub r#additional_pod_network_configs: Option<Vec<super::super::types::container::ClusterNodePoolNetworkConfigAdditionalPodNetworkConfig>>,
    /// Whether to create a new range for pod IPs in this node pool. Defaults are provided for `pod_range` and `pod_ipv4_cidr_block` if they are not specified.
    #[builder(into)]
    #[serde(rename = "createPodRange")]
    pub r#create_pod_range: Option<bool>,
    /// Whether nodes have internal IP addresses only.
    #[builder(into)]
    #[serde(rename = "enablePrivateNodes")]
    pub r#enable_private_nodes: Option<bool>,
    /// Network bandwidth tier configuration.
    #[builder(into)]
    #[serde(rename = "networkPerformanceConfig")]
    pub r#network_performance_config: Option<Box<super::super::types::container::ClusterNodePoolNetworkConfigNetworkPerformanceConfig>>,
    /// Configuration for node-pool level pod cidr overprovision. If not set, the cluster level setting will be inherited
    #[builder(into)]
    #[serde(rename = "podCidrOverprovisionConfig")]
    pub r#pod_cidr_overprovision_config: Option<Box<super::super::types::container::ClusterNodePoolNetworkConfigPodCidrOverprovisionConfig>>,
    /// The IP address range for pod IPs in this node pool. Only applicable if createPodRange is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) to pick a specific range to use.
    #[builder(into)]
    #[serde(rename = "podIpv4CidrBlock")]
    pub r#pod_ipv_4_cidr_block: Option<String>,
    /// The ID of the secondary range for pod IPs. If `create_pod_range` is true, this ID is used for the new range. If `create_pod_range` is false, uses an existing secondary range with this ID.
    #[builder(into)]
    #[serde(rename = "podRange")]
    pub r#pod_range: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterNodePoolNetworkConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "additional_node_network_configs",
                    &self.r#additional_node_network_configs,
                ),
                to_pulumi_object_field(
                    "additional_pod_network_configs",
                    &self.r#additional_pod_network_configs,
                ),
                to_pulumi_object_field(
                    "create_pod_range",
                    &self.r#create_pod_range,
                ),
                to_pulumi_object_field(
                    "enable_private_nodes",
                    &self.r#enable_private_nodes,
                ),
                to_pulumi_object_field(
                    "network_performance_config",
                    &self.r#network_performance_config,
                ),
                to_pulumi_object_field(
                    "pod_cidr_overprovision_config",
                    &self.r#pod_cidr_overprovision_config,
                ),
                to_pulumi_object_field(
                    "pod_ipv_4_cidr_block",
                    &self.r#pod_ipv_4_cidr_block,
                ),
                to_pulumi_object_field(
                    "pod_range",
                    &self.r#pod_range,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterNodePoolNetworkConfig {
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
                    r#additional_node_network_configs: {
                        let field_value = match fields_map.get("additional_node_network_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_node_network_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#additional_pod_network_configs: {
                        let field_value = match fields_map.get("additional_pod_network_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_pod_network_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_pod_range: {
                        let field_value = match fields_map.get("create_pod_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'create_pod_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_private_nodes: {
                        let field_value = match fields_map.get("enable_private_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_private_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_performance_config: {
                        let field_value = match fields_map.get("network_performance_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_performance_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_cidr_overprovision_config: {
                        let field_value = match fields_map.get("pod_cidr_overprovision_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'pod_cidr_overprovision_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_ipv_4_cidr_block: {
                        let field_value = match fields_map.get("pod_ipv_4_cidr_block") {
                            Some(value) => value,
                            None => bail!("Missing field 'pod_ipv_4_cidr_block' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_range: {
                        let field_value = match fields_map.get("pod_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'pod_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
