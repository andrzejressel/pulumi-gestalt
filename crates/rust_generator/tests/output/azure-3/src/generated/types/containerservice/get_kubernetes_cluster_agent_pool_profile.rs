#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetKubernetesClusterAgentPoolProfile {
    /// If the auto-scaler is enabled.
    #[builder(into)]
    #[serde(rename = "autoScalingEnabled")]
    pub r#auto_scaling_enabled: bool,
    /// The number of Agents (VMs) in the Pool.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: i32,
    /// Maximum number of nodes for auto-scaling
    #[builder(into)]
    #[serde(rename = "maxCount")]
    pub r#max_count: i32,
    /// The maximum number of pods that can run on each agent.
    #[builder(into)]
    #[serde(rename = "maxPods")]
    pub r#max_pods: i32,
    /// Minimum number of nodes for auto-scaling
    #[builder(into)]
    #[serde(rename = "minCount")]
    pub r#min_count: i32,
    /// The name of the managed Kubernetes Cluster.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "nodeLabels")]
    pub r#node_labels: std::collections::HashMap<String, String>,
    /// If the Public IPs for the nodes in this Agent Pool are enabled.
    #[builder(into)]
    #[serde(rename = "nodePublicIpEnabled")]
    pub r#node_public_ip_enabled: bool,
    /// Resource ID for the Public IP Addresses Prefix for the nodes in this Agent Pool.
    #[builder(into)]
    #[serde(rename = "nodePublicIpPrefixId")]
    pub r#node_public_ip_prefix_id: String,
    #[builder(into)]
    #[serde(rename = "nodeTaints")]
    pub r#node_taints: Vec<String>,
    /// Kubernetes version used for the Agents.
    #[builder(into)]
    #[serde(rename = "orchestratorVersion")]
    pub r#orchestrator_version: String,
    /// The size of the Agent VM's Operating System Disk in GB.
    #[builder(into)]
    #[serde(rename = "osDiskSizeGb")]
    pub r#os_disk_size_gb: i32,
    /// The Operating System used for the Agents.
    #[builder(into)]
    #[serde(rename = "osType")]
    pub r#os_type: String,
    /// A mapping of tags to assign to the resource.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: std::collections::HashMap<String, String>,
    /// The type of Managed Service Identity that is configured on this Kubernetes Cluster.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// A `upgrade_settings` block as documented below.
    #[builder(into)]
    #[serde(rename = "upgradeSettings")]
    pub r#upgrade_settings: Vec<super::super::types::containerservice::GetKubernetesClusterAgentPoolProfileUpgradeSetting>,
    /// The size of each VM in the Agent Pool (e.g. `Standard_F1`).
    #[builder(into)]
    #[serde(rename = "vmSize")]
    pub r#vm_size: String,
    /// The ID of the Subnet where the Agents in the Pool are provisioned.
    #[builder(into)]
    #[serde(rename = "vnetSubnetId")]
    pub r#vnet_subnet_id: String,
    /// A list of Availability Zones in which this Kubernetes Cluster is located.
    #[builder(into)]
    #[serde(rename = "zones")]
    pub r#zones: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetKubernetesClusterAgentPoolProfile {
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
                    "auto_scaling_enabled",
                    &self.r#auto_scaling_enabled,
                ),
                to_pulumi_object_field(
                    "count",
                    &self.r#count,
                ),
                to_pulumi_object_field(
                    "max_count",
                    &self.r#max_count,
                ),
                to_pulumi_object_field(
                    "max_pods",
                    &self.r#max_pods,
                ),
                to_pulumi_object_field(
                    "min_count",
                    &self.r#min_count,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "node_labels",
                    &self.r#node_labels,
                ),
                to_pulumi_object_field(
                    "node_public_ip_enabled",
                    &self.r#node_public_ip_enabled,
                ),
                to_pulumi_object_field(
                    "node_public_ip_prefix_id",
                    &self.r#node_public_ip_prefix_id,
                ),
                to_pulumi_object_field(
                    "node_taints",
                    &self.r#node_taints,
                ),
                to_pulumi_object_field(
                    "orchestrator_version",
                    &self.r#orchestrator_version,
                ),
                to_pulumi_object_field(
                    "os_disk_size_gb",
                    &self.r#os_disk_size_gb,
                ),
                to_pulumi_object_field(
                    "os_type",
                    &self.r#os_type,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
                to_pulumi_object_field(
                    "upgrade_settings",
                    &self.r#upgrade_settings,
                ),
                to_pulumi_object_field(
                    "vm_size",
                    &self.r#vm_size,
                ),
                to_pulumi_object_field(
                    "vnet_subnet_id",
                    &self.r#vnet_subnet_id,
                ),
                to_pulumi_object_field(
                    "zones",
                    &self.r#zones,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetKubernetesClusterAgentPoolProfile {
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
                    r#auto_scaling_enabled: {
                        let field_value = match fields_map.get("auto_scaling_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_scaling_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#count: {
                        let field_value = match fields_map.get("count") {
                            Some(value) => value,
                            None => bail!("Missing field 'count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_count: {
                        let field_value = match fields_map.get("max_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_pods: {
                        let field_value = match fields_map.get("max_pods") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_pods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_count: {
                        let field_value = match fields_map.get("min_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#node_labels: {
                        let field_value = match fields_map.get("node_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_public_ip_enabled: {
                        let field_value = match fields_map.get("node_public_ip_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_public_ip_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_public_ip_prefix_id: {
                        let field_value = match fields_map.get("node_public_ip_prefix_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_public_ip_prefix_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_taints: {
                        let field_value = match fields_map.get("node_taints") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_taints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#orchestrator_version: {
                        let field_value = match fields_map.get("orchestrator_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'orchestrator_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_disk_size_gb: {
                        let field_value = match fields_map.get("os_disk_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_disk_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_type: {
                        let field_value = match fields_map.get("os_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upgrade_settings: {
                        let field_value = match fields_map.get("upgrade_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'upgrade_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_size: {
                        let field_value = match fields_map.get("vm_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vnet_subnet_id: {
                        let field_value = match fields_map.get("vnet_subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vnet_subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zones: {
                        let field_value = match fields_map.get("zones") {
                            Some(value) => value,
                            None => bail!("Missing field 'zones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
