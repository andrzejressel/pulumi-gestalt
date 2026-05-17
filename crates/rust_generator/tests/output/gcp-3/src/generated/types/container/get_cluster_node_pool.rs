#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterNodePool {
    /// Configuration required by cluster autoscaler to adjust the size of the node pool to the current cluster usage.
    #[builder(into)]
    #[serde(rename = "autoscalings")]
    pub r#autoscalings: Vec<super::super::types::container::GetClusterNodePoolAutoscaling>,
    /// The initial number of nodes for the pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Changing this will force recreation of the resource.
    #[builder(into)]
    #[serde(rename = "initialNodeCount")]
    pub r#initial_node_count: i32,
    /// The resource URLs of the managed instance groups associated with this node pool.
    #[builder(into)]
    #[serde(rename = "instanceGroupUrls")]
    pub r#instance_group_urls: Vec<String>,
    /// List of instance group URLs which have been assigned to this node pool.
    #[builder(into)]
    #[serde(rename = "managedInstanceGroupUrls")]
    pub r#managed_instance_group_urls: Vec<String>,
    /// Node management configuration, wherein auto-repair and auto-upgrade is configured.
    #[builder(into)]
    #[serde(rename = "managements")]
    pub r#managements: Vec<super::super::types::container::GetClusterNodePoolManagement>,
    /// The maximum number of pods per node in this node pool. Note that this does not work on node pools which are "route-based" - that is, node pools belonging to clusters that do not have IP Aliasing enabled.
    #[builder(into)]
    #[serde(rename = "maxPodsPerNode")]
    pub r#max_pods_per_node: i32,
    /// The name of the cluster.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Creates a unique name for the node pool beginning with the specified prefix. Conflicts with name.
    #[builder(into)]
    #[serde(rename = "namePrefix")]
    pub r#name_prefix: String,
    /// Networking configuration for this NodePool. If specified, it overrides the cluster-level defaults.
    #[builder(into)]
    #[serde(rename = "networkConfigs")]
    pub r#network_configs: Vec<super::super::types::container::GetClusterNodePoolNetworkConfig>,
    /// The configuration of the nodepool
    #[builder(into)]
    #[serde(rename = "nodeConfigs")]
    pub r#node_configs: Vec<super::super::types::container::GetClusterNodePoolNodeConfig>,
    /// The number of nodes per instance group. This field can be used to update the number of nodes per instance group but should not be used alongside autoscaling.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: i32,
    /// The list of zones in which the node pool's nodes should be located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If unspecified, the cluster-level node_locations will be used.
    #[builder(into)]
    #[serde(rename = "nodeLocations")]
    pub r#node_locations: Vec<String>,
    /// Specifies the node placement policy
    #[builder(into)]
    #[serde(rename = "placementPolicies")]
    pub r#placement_policies: Vec<super::super::types::container::GetClusterNodePoolPlacementPolicy>,
    /// Specifies the configuration of queued provisioning
    #[builder(into)]
    #[serde(rename = "queuedProvisionings")]
    pub r#queued_provisionings: Vec<super::super::types::container::GetClusterNodePoolQueuedProvisioning>,
    /// Specify node upgrade settings to change how many nodes GKE attempts to upgrade at once. The number of nodes upgraded simultaneously is the sum of max_surge and max_unavailable. The maximum number of nodes upgraded simultaneously is limited to 20.
    #[builder(into)]
    #[serde(rename = "upgradeSettings")]
    pub r#upgrade_settings: Vec<super::super::types::container::GetClusterNodePoolUpgradeSetting>,
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterNodePool {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "autoscalings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#autoscalings,
                )
                .await,
            );
            map.insert(
                "initial_node_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#initial_node_count,
                )
                .await,
            );
            map.insert(
                "instance_group_urls".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_group_urls,
                )
                .await,
            );
            map.insert(
                "managed_instance_group_urls".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#managed_instance_group_urls,
                )
                .await,
            );
            map.insert(
                "managements".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#managements,
                )
                .await,
            );
            map.insert(
                "max_pods_per_node".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_pods_per_node,
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
                "name_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name_prefix,
                )
                .await,
            );
            map.insert(
                "network_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_configs,
                )
                .await,
            );
            map.insert(
                "node_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_configs,
                )
                .await,
            );
            map.insert(
                "node_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_count,
                )
                .await,
            );
            map.insert(
                "node_locations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_locations,
                )
                .await,
            );
            map.insert(
                "placement_policies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#placement_policies,
                )
                .await,
            );
            map.insert(
                "queued_provisionings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#queued_provisionings,
                )
                .await,
            );
            map.insert(
                "upgrade_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#upgrade_settings,
                )
                .await,
            );
            map.insert(
                "version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#version,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterNodePool {
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
                    r#autoscalings: {
                        let field_value = match fields_map.get("autoscalings") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoscalings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#initial_node_count: {
                        let field_value = match fields_map.get("initial_node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'initial_node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_group_urls: {
                        let field_value = match fields_map.get("instance_group_urls") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_group_urls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_instance_group_urls: {
                        let field_value = match fields_map.get("managed_instance_group_urls") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_instance_group_urls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managements: {
                        let field_value = match fields_map.get("managements") {
                            Some(value) => value,
                            None => bail!("Missing field 'managements' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_pods_per_node: {
                        let field_value = match fields_map.get("max_pods_per_node") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_pods_per_node' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#name_prefix: {
                        let field_value = match fields_map.get("name_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'name_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_configs: {
                        let field_value = match fields_map.get("network_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_configs: {
                        let field_value = match fields_map.get("node_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_count: {
                        let field_value = match fields_map.get("node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_locations: {
                        let field_value = match fields_map.get("node_locations") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_locations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#placement_policies: {
                        let field_value = match fields_map.get("placement_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'placement_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#queued_provisionings: {
                        let field_value = match fields_map.get("queued_provisionings") {
                            Some(value) => value,
                            None => bail!("Missing field 'queued_provisionings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
