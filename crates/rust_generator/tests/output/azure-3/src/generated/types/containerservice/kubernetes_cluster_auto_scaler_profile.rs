#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterAutoScalerProfile {
    /// Detect similar node groups and balance the number of nodes between them. Defaults to `false`.
    #[builder(into)]
    pub r#balance_similar_node_groups: Option<bool>,
    /// Whether DaemonSet pods will be gracefully terminated from empty nodes. Defaults to `false`.
    #[builder(into)]
    pub r#daemonset_eviction_for_empty_nodes_enabled: Option<bool>,
    /// Whether DaemonSet pods will be gracefully terminated from non-empty nodes. Defaults to `true`.
    #[builder(into)]
    pub r#daemonset_eviction_for_occupied_nodes_enabled: Option<bool>,
    /// Maximum number of empty nodes that can be deleted at the same time. Defaults to `10`.
    #[builder(into)]
    pub r#empty_bulk_delete_max: Option<String>,
    /// Expander to use. Possible values are `least-waste`, `priority`, `most-pods` and `random`. Defaults to `random`.
    #[builder(into)]
    pub r#expander: Option<String>,
    /// Whether DaemonSet pods will be ignored when calculating resource utilization for scale down. Defaults to `false`.
    #[builder(into)]
    pub r#ignore_daemonsets_utilization_enabled: Option<bool>,
    /// Maximum number of seconds the cluster autoscaler waits for pod termination when trying to scale down a node. Defaults to `600`.
    #[builder(into)]
    pub r#max_graceful_termination_sec: Option<String>,
    /// Maximum time the autoscaler waits for a node to be provisioned. Defaults to `15m`.
    #[builder(into)]
    pub r#max_node_provisioning_time: Option<String>,
    /// Maximum Number of allowed unready nodes. Defaults to `3`.
    #[builder(into)]
    pub r#max_unready_nodes: Option<i32>,
    /// Maximum percentage of unready nodes the cluster autoscaler will stop if the percentage is exceeded. Defaults to `45`.
    #[builder(into)]
    pub r#max_unready_percentage: Option<f64>,
    /// For scenarios like burst/batch scale where you don't want CA to act before the kubernetes scheduler could schedule all the pods, you can tell CA to ignore unscheduled pods before they're a certain age. Defaults to `10s`.
    #[builder(into)]
    pub r#new_pod_scale_up_delay: Option<String>,
    /// How long after the scale up of AKS nodes the scale down evaluation resumes. Defaults to `10m`.
    #[builder(into)]
    pub r#scale_down_delay_after_add: Option<String>,
    /// How long after node deletion that scale down evaluation resumes. Defaults to the value used for `scan_interval`.
    #[builder(into)]
    pub r#scale_down_delay_after_delete: Option<String>,
    /// How long after scale down failure that scale down evaluation resumes. Defaults to `3m`.
    #[builder(into)]
    pub r#scale_down_delay_after_failure: Option<String>,
    /// How long a node should be unneeded before it is eligible for scale down. Defaults to `10m`.
    #[builder(into)]
    pub r#scale_down_unneeded: Option<String>,
    /// How long an unready node should be unneeded before it is eligible for scale down. Defaults to `20m`.
    #[builder(into)]
    pub r#scale_down_unready: Option<String>,
    /// Node utilization level, defined as sum of requested resources divided by capacity, below which a node can be considered for scale down. Defaults to `0.5`.
    #[builder(into)]
    pub r#scale_down_utilization_threshold: Option<String>,
    /// How often the AKS Cluster should be re-evaluated for scale up/down. Defaults to `10s`.
    #[builder(into)]
    pub r#scan_interval: Option<String>,
    /// If `true` cluster autoscaler will never delete nodes with pods with local storage, for example, EmptyDir or HostPath. Defaults to `true`.
    #[builder(into)]
    pub r#skip_nodes_with_local_storage: Option<bool>,
    /// If `true` cluster autoscaler will never delete nodes with pods from kube-system (except for DaemonSet or mirror pods). Defaults to `true`.
    #[builder(into)]
    pub r#skip_nodes_with_system_pods: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterAutoScalerProfile {
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
                    "balanceSimilarNodeGroups",
                    &self.r#balance_similar_node_groups,
                ),
                to_pulumi_object_field(
                    "daemonsetEvictionForEmptyNodesEnabled",
                    &self.r#daemonset_eviction_for_empty_nodes_enabled,
                ),
                to_pulumi_object_field(
                    "daemonsetEvictionForOccupiedNodesEnabled",
                    &self.r#daemonset_eviction_for_occupied_nodes_enabled,
                ),
                to_pulumi_object_field(
                    "emptyBulkDeleteMax",
                    &self.r#empty_bulk_delete_max,
                ),
                to_pulumi_object_field(
                    "expander",
                    &self.r#expander,
                ),
                to_pulumi_object_field(
                    "ignoreDaemonsetsUtilizationEnabled",
                    &self.r#ignore_daemonsets_utilization_enabled,
                ),
                to_pulumi_object_field(
                    "maxGracefulTerminationSec",
                    &self.r#max_graceful_termination_sec,
                ),
                to_pulumi_object_field(
                    "maxNodeProvisioningTime",
                    &self.r#max_node_provisioning_time,
                ),
                to_pulumi_object_field(
                    "maxUnreadyNodes",
                    &self.r#max_unready_nodes,
                ),
                to_pulumi_object_field(
                    "maxUnreadyPercentage",
                    &self.r#max_unready_percentage,
                ),
                to_pulumi_object_field(
                    "newPodScaleUpDelay",
                    &self.r#new_pod_scale_up_delay,
                ),
                to_pulumi_object_field(
                    "scaleDownDelayAfterAdd",
                    &self.r#scale_down_delay_after_add,
                ),
                to_pulumi_object_field(
                    "scaleDownDelayAfterDelete",
                    &self.r#scale_down_delay_after_delete,
                ),
                to_pulumi_object_field(
                    "scaleDownDelayAfterFailure",
                    &self.r#scale_down_delay_after_failure,
                ),
                to_pulumi_object_field(
                    "scaleDownUnneeded",
                    &self.r#scale_down_unneeded,
                ),
                to_pulumi_object_field(
                    "scaleDownUnready",
                    &self.r#scale_down_unready,
                ),
                to_pulumi_object_field(
                    "scaleDownUtilizationThreshold",
                    &self.r#scale_down_utilization_threshold,
                ),
                to_pulumi_object_field(
                    "scanInterval",
                    &self.r#scan_interval,
                ),
                to_pulumi_object_field(
                    "skipNodesWithLocalStorage",
                    &self.r#skip_nodes_with_local_storage,
                ),
                to_pulumi_object_field(
                    "skipNodesWithSystemPods",
                    &self.r#skip_nodes_with_system_pods,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterAutoScalerProfile {
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
                    r#balance_similar_node_groups: {
                        let field_value = match fields_map.get("balanceSimilarNodeGroups") {
                            Some(value) => value,
                            None => bail!("Missing field 'balanceSimilarNodeGroups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#daemonset_eviction_for_empty_nodes_enabled: {
                        let field_value = match fields_map.get("daemonsetEvictionForEmptyNodesEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'daemonsetEvictionForEmptyNodesEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#daemonset_eviction_for_occupied_nodes_enabled: {
                        let field_value = match fields_map.get("daemonsetEvictionForOccupiedNodesEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'daemonsetEvictionForOccupiedNodesEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#empty_bulk_delete_max: {
                        let field_value = match fields_map.get("emptyBulkDeleteMax") {
                            Some(value) => value,
                            None => bail!("Missing field 'emptyBulkDeleteMax' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expander: {
                        let field_value = match fields_map.get("expander") {
                            Some(value) => value,
                            None => bail!("Missing field 'expander' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_daemonsets_utilization_enabled: {
                        let field_value = match fields_map.get("ignoreDaemonsetsUtilizationEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignoreDaemonsetsUtilizationEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_graceful_termination_sec: {
                        let field_value = match fields_map.get("maxGracefulTerminationSec") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxGracefulTerminationSec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_node_provisioning_time: {
                        let field_value = match fields_map.get("maxNodeProvisioningTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxNodeProvisioningTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_unready_nodes: {
                        let field_value = match fields_map.get("maxUnreadyNodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxUnreadyNodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_unready_percentage: {
                        let field_value = match fields_map.get("maxUnreadyPercentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxUnreadyPercentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#new_pod_scale_up_delay: {
                        let field_value = match fields_map.get("newPodScaleUpDelay") {
                            Some(value) => value,
                            None => bail!("Missing field 'newPodScaleUpDelay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_down_delay_after_add: {
                        let field_value = match fields_map.get("scaleDownDelayAfterAdd") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaleDownDelayAfterAdd' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_down_delay_after_delete: {
                        let field_value = match fields_map.get("scaleDownDelayAfterDelete") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaleDownDelayAfterDelete' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_down_delay_after_failure: {
                        let field_value = match fields_map.get("scaleDownDelayAfterFailure") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaleDownDelayAfterFailure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_down_unneeded: {
                        let field_value = match fields_map.get("scaleDownUnneeded") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaleDownUnneeded' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_down_unready: {
                        let field_value = match fields_map.get("scaleDownUnready") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaleDownUnready' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_down_utilization_threshold: {
                        let field_value = match fields_map.get("scaleDownUtilizationThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaleDownUtilizationThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scan_interval: {
                        let field_value = match fields_map.get("scanInterval") {
                            Some(value) => value,
                            None => bail!("Missing field 'scanInterval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#skip_nodes_with_local_storage: {
                        let field_value = match fields_map.get("skipNodesWithLocalStorage") {
                            Some(value) => value,
                            None => bail!("Missing field 'skipNodesWithLocalStorage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#skip_nodes_with_system_pods: {
                        let field_value = match fields_map.get("skipNodesWithSystemPods") {
                            Some(value) => value,
                            None => bail!("Missing field 'skipNodesWithSystemPods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
