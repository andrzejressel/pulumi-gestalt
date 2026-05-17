#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterAddonsConfig {
    /// . Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cloudrunConfig")]
    pub r#cloudrun_config: Option<Box<super::super::types::container::ClusterAddonsConfigCloudrunConfig>>,
    /// .
    /// The status of the ConfigConnector addon. It is disabled by default; Set `enabled = true` to enable.
    #[builder(into)]
    #[serde(rename = "configConnectorConfig")]
    pub r#config_connector_config: Option<Box<super::super::types::container::ClusterAddonsConfigConfigConnectorConfig>>,
    /// .
    /// The status of the NodeLocal DNSCache addon. It is disabled by default.
    /// Set `enabled = true` to enable.
    /// 
    /// **Enabling/Disabling NodeLocal DNSCache in an existing cluster is a disruptive operation.
    /// All cluster nodes running GKE 1.15 and higher are recreated.**
    #[builder(into)]
    #[serde(rename = "dnsCacheConfig")]
    pub r#dns_cache_config: Option<Box<super::super::types::container::ClusterAddonsConfigDnsCacheConfig>>,
    /// .
    /// Whether this cluster should enable the Google Compute Engine Persistent Disk Container Storage Interface (CSI) Driver. Set `enabled = true` to enable.
    /// 
    /// **Note:** The Compute Engine persistent disk CSI Driver is enabled by default on newly created clusters for the following versions: Linux clusters: GKE version 1.18.10-gke.2100 or later, or 1.19.3-gke.2100 or later.
    #[builder(into)]
    #[serde(rename = "gcePersistentDiskCsiDriverConfig")]
    pub r#gce_persistent_disk_csi_driver_config: Option<Box<super::super::types::container::ClusterAddonsConfigGcePersistentDiskCsiDriverConfig>>,
    /// The status of the Filestore CSI driver addon,
    /// which allows the usage of filestore instance as volumes.
    /// It is disabled by default; set `enabled = true` to enable.
    #[builder(into)]
    #[serde(rename = "gcpFilestoreCsiDriverConfig")]
    pub r#gcp_filestore_csi_driver_config: Option<Box<super::super::types::container::ClusterAddonsConfigGcpFilestoreCsiDriverConfig>>,
    /// The status of the GCSFuse CSI driver addon,
    /// which allows the usage of a gcs bucket as volumes.
    /// It is disabled by default for Standard clusters; set `enabled = true` to enable.
    /// It is enabled by default for Autopilot clusters with version 1.24 or later; set `enabled = true` to enable it explicitly.
    /// See [Enable the Cloud Storage FUSE CSI driver](https://cloud.google.com/kubernetes-engine/docs/how-to/persistent-volumes/cloud-storage-fuse-csi-driver#enable) for more information.
    #[builder(into)]
    #[serde(rename = "gcsFuseCsiDriverConfig")]
    pub r#gcs_fuse_csi_driver_config: Option<Box<super::super::types::container::ClusterAddonsConfigGcsFuseCsiDriverConfig>>,
    /// .
    /// The status of the Backup for GKE agent addon. It is disabled by default; Set `enabled = true` to enable.
    #[builder(into)]
    #[serde(rename = "gkeBackupAgentConfig")]
    pub r#gke_backup_agent_config: Option<Box<super::super::types::container::ClusterAddonsConfigGkeBackupAgentConfig>>,
    /// The status of the Horizontal Pod Autoscaling
    /// addon, which increases or decreases the number of replica pods a replication controller
    /// has based on the resource usage of the existing pods.
    /// It is enabled by default;
    /// set `disabled = true` to disable.
    #[builder(into)]
    #[serde(rename = "horizontalPodAutoscaling")]
    pub r#horizontal_pod_autoscaling: Option<Box<super::super::types::container::ClusterAddonsConfigHorizontalPodAutoscaling>>,
    /// The status of the HTTP (L7) load balancing
    /// controller addon, which makes it easy to set up HTTP load balancers for services in a
    /// cluster. It is enabled by default; set `disabled = true` to disable.
    #[builder(into)]
    #[serde(rename = "httpLoadBalancing")]
    pub r#http_load_balancing: Option<Box<super::super::types::container::ClusterAddonsConfigHttpLoadBalancing>>,
    /// .
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "istioConfig")]
    pub r#istio_config: Option<Box<super::super::types::container::ClusterAddonsConfigIstioConfig>>,
    /// .
    /// Configuration for the KALM addon, which manages the lifecycle of k8s. It is disabled by default; Set `enabled = true` to enable.
    #[builder(into)]
    #[serde(rename = "kalmConfig")]
    pub r#kalm_config: Option<Box<super::super::types::container::ClusterAddonsConfigKalmConfig>>,
    /// Whether we should enable the network policy addon
    /// for the master.  This must be enabled in order to enable network policy for the nodes.
    /// To enable this, you must also define a `network_policy` block,
    /// otherwise nothing will happen.
    /// It can only be disabled if the nodes already do not have network policies enabled.
    /// Defaults to disabled; set `disabled = false` to enable.
    #[builder(into)]
    #[serde(rename = "networkPolicyConfig")]
    pub r#network_policy_config: Option<Box<super::super::types::container::ClusterAddonsConfigNetworkPolicyConfig>>,
    /// The status of the Parallelstore CSI driver addon,
    /// which allows the usage of a Parallelstore instances as volumes.
    /// It is disabled by default for Standard clusters; set `enabled = true` to enable.
    /// It is enabled by default for Autopilot clusters with version 1.29 or later; set `enabled = true` to enable it explicitly.
    /// See [Enable the Parallelstore CSI driver](https://cloud.google.com/kubernetes-engine/docs/how-to/persistent-volumes/parallelstore-csi-new-volume#enable) for more information.
    /// 
    /// This example `addons_config` disables two addons:
    /// 
    #[builder(into)]
    #[serde(rename = "parallelstoreCsiDriverConfig")]
    pub r#parallelstore_csi_driver_config: Option<Box<super::super::types::container::ClusterAddonsConfigParallelstoreCsiDriverConfig>>,
    /// . The status of the [Ray Operator
    /// addon](https://cloud.google.com/kubernetes-engine/docs/add-on/ray-on-gke/concepts/overview).
    /// It is disabled by default. Set `enabled = true` to enable. The minimum
    /// cluster version to enable Ray is 1.30.0-gke.1747000.
    /// 
    /// Ray Operator config has optional subfields
    /// `ray_cluster_logging_config.enabled` and
    /// `ray_cluster_monitoring_config.enabled` which control Ray Cluster logging
    /// and monitoring respectively. See [Collect and view logs and metrics for Ray
    /// clusters on
    /// GKE](https://cloud.google.com/kubernetes-engine/docs/add-on/ray-on-gke/how-to/collect-view-logs-metrics)
    /// for more information.
    #[builder(into)]
    #[serde(rename = "rayOperatorConfigs")]
    pub r#ray_operator_configs: Option<Vec<super::super::types::container::ClusterAddonsConfigRayOperatorConfig>>,
    /// .
    /// The status of the Stateful HA addon, which provides automatic configurable failover for stateful applications.
    /// It is disabled by default for Standard clusters. Set `enabled = true` to enable.
    #[builder(into)]
    #[serde(rename = "statefulHaConfig")]
    pub r#stateful_ha_config: Option<Box<super::super::types::container::ClusterAddonsConfigStatefulHaConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterAddonsConfig {
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
                    "cloudrun_config",
                    &self.r#cloudrun_config,
                ),
                to_pulumi_object_field(
                    "config_connector_config",
                    &self.r#config_connector_config,
                ),
                to_pulumi_object_field(
                    "dns_cache_config",
                    &self.r#dns_cache_config,
                ),
                to_pulumi_object_field(
                    "gce_persistent_disk_csi_driver_config",
                    &self.r#gce_persistent_disk_csi_driver_config,
                ),
                to_pulumi_object_field(
                    "gcp_filestore_csi_driver_config",
                    &self.r#gcp_filestore_csi_driver_config,
                ),
                to_pulumi_object_field(
                    "gcs_fuse_csi_driver_config",
                    &self.r#gcs_fuse_csi_driver_config,
                ),
                to_pulumi_object_field(
                    "gke_backup_agent_config",
                    &self.r#gke_backup_agent_config,
                ),
                to_pulumi_object_field(
                    "horizontal_pod_autoscaling",
                    &self.r#horizontal_pod_autoscaling,
                ),
                to_pulumi_object_field(
                    "http_load_balancing",
                    &self.r#http_load_balancing,
                ),
                to_pulumi_object_field(
                    "istio_config",
                    &self.r#istio_config,
                ),
                to_pulumi_object_field(
                    "kalm_config",
                    &self.r#kalm_config,
                ),
                to_pulumi_object_field(
                    "network_policy_config",
                    &self.r#network_policy_config,
                ),
                to_pulumi_object_field(
                    "parallelstore_csi_driver_config",
                    &self.r#parallelstore_csi_driver_config,
                ),
                to_pulumi_object_field(
                    "ray_operator_configs",
                    &self.r#ray_operator_configs,
                ),
                to_pulumi_object_field(
                    "stateful_ha_config",
                    &self.r#stateful_ha_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterAddonsConfig {
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
                    r#cloudrun_config: {
                        let field_value = match fields_map.get("cloudrun_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudrun_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#config_connector_config: {
                        let field_value = match fields_map.get("config_connector_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'config_connector_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_cache_config: {
                        let field_value = match fields_map.get("dns_cache_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_cache_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gce_persistent_disk_csi_driver_config: {
                        let field_value = match fields_map.get("gce_persistent_disk_csi_driver_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'gce_persistent_disk_csi_driver_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcp_filestore_csi_driver_config: {
                        let field_value = match fields_map.get("gcp_filestore_csi_driver_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcp_filestore_csi_driver_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcs_fuse_csi_driver_config: {
                        let field_value = match fields_map.get("gcs_fuse_csi_driver_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcs_fuse_csi_driver_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gke_backup_agent_config: {
                        let field_value = match fields_map.get("gke_backup_agent_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'gke_backup_agent_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#horizontal_pod_autoscaling: {
                        let field_value = match fields_map.get("horizontal_pod_autoscaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'horizontal_pod_autoscaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_load_balancing: {
                        let field_value = match fields_map.get("http_load_balancing") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_load_balancing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#istio_config: {
                        let field_value = match fields_map.get("istio_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'istio_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kalm_config: {
                        let field_value = match fields_map.get("kalm_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'kalm_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_policy_config: {
                        let field_value = match fields_map.get("network_policy_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_policy_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parallelstore_csi_driver_config: {
                        let field_value = match fields_map.get("parallelstore_csi_driver_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'parallelstore_csi_driver_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ray_operator_configs: {
                        let field_value = match fields_map.get("ray_operator_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'ray_operator_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateful_ha_config: {
                        let field_value = match fields_map.get("stateful_ha_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'stateful_ha_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
