#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNodePool {
    /// Configuration required by cluster autoscaler to adjust the size of the node pool to the current cluster usage.
    #[builder(into)]
    #[serde(rename = "autoscaling")]
    pub r#autoscaling: Option<Box<super::super::types::container::ClusterNodePoolAutoscaling>>,
    /// The number of nodes to create in this
    /// cluster's default node pool. In regional or multi-zonal clusters, this is the
    /// number of nodes per zone. Must be set if `node_pool` is not set. If you're using
    /// `gcp.container.NodePool` objects with no default node pool, you'll need to
    /// set this to a value of at least `1`, alongside setting
    /// `remove_default_node_pool` to `true`.
    #[builder(into)]
    #[serde(rename = "initialNodeCount")]
    pub r#initial_node_count: Option<i32>,
    /// The resource URLs of the managed instance groups associated with this node pool.
    #[builder(into)]
    #[serde(rename = "instanceGroupUrls")]
    pub r#instance_group_urls: Option<Vec<String>>,
    /// List of instance group URLs which have been assigned to this node pool.
    #[builder(into)]
    #[serde(rename = "managedInstanceGroupUrls")]
    pub r#managed_instance_group_urls: Option<Vec<String>>,
    /// Node management configuration, wherein auto-repair and auto-upgrade is configured.
    #[builder(into)]
    #[serde(rename = "management")]
    pub r#management: Option<Box<super::super::types::container::ClusterNodePoolManagement>>,
    /// The maximum number of pods per node in this node pool. Note that this does not work on node pools which are "route-based" - that is, node pools belonging to clusters that do not have IP Aliasing enabled.
    #[builder(into)]
    #[serde(rename = "maxPodsPerNode")]
    pub r#max_pods_per_node: Option<i32>,
    /// The name of the cluster, unique within the project and
    /// location.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Creates a unique name for the node pool beginning with the specified prefix. Conflicts with name.
    #[builder(into)]
    #[serde(rename = "namePrefix")]
    pub r#name_prefix: Option<String>,
    /// Configuration for
    /// [Adding Pod IP address ranges](https://cloud.google.com/kubernetes-engine/docs/how-to/multi-pod-cidr)) to the node pool. Structure is documented below
    #[builder(into)]
    #[serde(rename = "networkConfig")]
    pub r#network_config: Option<Box<super::super::types::container::ClusterNodePoolNetworkConfig>>,
    /// Parameters used in creating the default node pool.
    /// Generally, this field should not be used at the same time as a
    /// `gcp.container.NodePool` or a `node_pool` block; this configuration
    /// manages the default node pool, which isn't recommended to be used.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "nodeConfig")]
    pub r#node_config: Option<Box<super::super::types::container::ClusterNodePoolNodeConfig>>,
    /// The number of nodes per instance group. This field can be used to update the number of nodes per instance group but should not be used alongside autoscaling.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Option<i32>,
    /// The list of zones in which the cluster's nodes
    /// are located. Nodes must be in the region of their regional cluster or in the
    /// same region as their cluster's zone for zonal clusters. If this is specified for
    /// a zonal cluster, omit the cluster's zone.
    /// 
    /// > A "multi-zonal" cluster is a zonal cluster with at least one additional zone
    /// defined; in a multi-zonal cluster, the cluster master is only present in a
    /// single zone while nodes are present in each of the primary zone and the node
    /// locations. In contrast, in a regional cluster, cluster master nodes are present
    /// in multiple zones in the region. For that reason, regional clusters should be
    /// preferred.
    #[builder(into)]
    #[serde(rename = "nodeLocations")]
    pub r#node_locations: Option<Vec<String>>,
    /// Specifies the node placement policy
    #[builder(into)]
    #[serde(rename = "placementPolicy")]
    pub r#placement_policy: Option<Box<super::super::types::container::ClusterNodePoolPlacementPolicy>>,
    /// Specifies the configuration of queued provisioning
    #[builder(into)]
    #[serde(rename = "queuedProvisioning")]
    pub r#queued_provisioning: Option<Box<super::super::types::container::ClusterNodePoolQueuedProvisioning>>,
    /// Specify node upgrade settings to change how many nodes GKE attempts to upgrade at once. The number of nodes upgraded simultaneously is the sum of max_surge and max_unavailable. The maximum number of nodes upgraded simultaneously is limited to 20.
    #[builder(into)]
    #[serde(rename = "upgradeSettings")]
    pub r#upgrade_settings: Option<Box<super::super::types::container::ClusterNodePoolUpgradeSettings>>,
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterNodePool {
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
                    "autoscaling",
                    &self.r#autoscaling,
                ),
                to_pulumi_object_field(
                    "initial_node_count",
                    &self.r#initial_node_count,
                ),
                to_pulumi_object_field(
                    "instance_group_urls",
                    &self.r#instance_group_urls,
                ),
                to_pulumi_object_field(
                    "managed_instance_group_urls",
                    &self.r#managed_instance_group_urls,
                ),
                to_pulumi_object_field(
                    "management",
                    &self.r#management,
                ),
                to_pulumi_object_field(
                    "max_pods_per_node",
                    &self.r#max_pods_per_node,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "name_prefix",
                    &self.r#name_prefix,
                ),
                to_pulumi_object_field(
                    "network_config",
                    &self.r#network_config,
                ),
                to_pulumi_object_field(
                    "node_config",
                    &self.r#node_config,
                ),
                to_pulumi_object_field(
                    "node_count",
                    &self.r#node_count,
                ),
                to_pulumi_object_field(
                    "node_locations",
                    &self.r#node_locations,
                ),
                to_pulumi_object_field(
                    "placement_policy",
                    &self.r#placement_policy,
                ),
                to_pulumi_object_field(
                    "queued_provisioning",
                    &self.r#queued_provisioning,
                ),
                to_pulumi_object_field(
                    "upgrade_settings",
                    &self.r#upgrade_settings,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterNodePool {
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
                    r#autoscaling: {
                        let field_value = match fields_map.get("autoscaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoscaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#management: {
                        let field_value = match fields_map.get("management") {
                            Some(value) => value,
                            None => bail!("Missing field 'management' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#network_config: {
                        let field_value = match fields_map.get("network_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_config: {
                        let field_value = match fields_map.get("node_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#placement_policy: {
                        let field_value = match fields_map.get("placement_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'placement_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#queued_provisioning: {
                        let field_value = match fields_map.get("queued_provisioning") {
                            Some(value) => value,
                            None => bail!("Missing field 'queued_provisioning' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
