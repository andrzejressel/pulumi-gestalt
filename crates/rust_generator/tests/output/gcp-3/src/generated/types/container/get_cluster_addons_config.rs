#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterAddonsConfig {
    /// The status of the CloudRun addon. It is disabled by default. Set disabled = false to enable.
    #[builder(into)]
    #[serde(rename = "cloudrunConfigs")]
    pub r#cloudrun_configs: Vec<super::super::types::container::GetClusterAddonsConfigCloudrunConfig>,
    /// The of the Config Connector addon.
    #[builder(into)]
    #[serde(rename = "configConnectorConfigs")]
    pub r#config_connector_configs: Vec<super::super::types::container::GetClusterAddonsConfigConfigConnectorConfig>,
    /// The status of the NodeLocal DNSCache addon. It is disabled by default. Set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "dnsCacheConfigs")]
    pub r#dns_cache_configs: Vec<super::super::types::container::GetClusterAddonsConfigDnsCacheConfig>,
    /// Whether this cluster should enable the Google Compute Engine Persistent Disk Container Storage Interface (CSI) Driver. Set enabled = true to enable. The Compute Engine persistent disk CSI Driver is enabled by default on newly created clusters for the following versions: Linux clusters: GKE version 1.18.10-gke.2100 or later, or 1.19.3-gke.2100 or later.
    #[builder(into)]
    #[serde(rename = "gcePersistentDiskCsiDriverConfigs")]
    pub r#gce_persistent_disk_csi_driver_configs: Vec<super::super::types::container::GetClusterAddonsConfigGcePersistentDiskCsiDriverConfig>,
    /// The status of the Filestore CSI driver addon, which allows the usage of filestore instance as volumes. Defaults to disabled for Standard clusters; set enabled = true to enable. It is enabled by default for Autopilot clusters; set enabled = true to enable it explicitly.
    #[builder(into)]
    #[serde(rename = "gcpFilestoreCsiDriverConfigs")]
    pub r#gcp_filestore_csi_driver_configs: Vec<super::super::types::container::GetClusterAddonsConfigGcpFilestoreCsiDriverConfig>,
    /// The status of the GCS Fuse CSI driver addon, which allows the usage of gcs bucket as volumes. Defaults to disabled; set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "gcsFuseCsiDriverConfigs")]
    pub r#gcs_fuse_csi_driver_configs: Vec<super::super::types::container::GetClusterAddonsConfigGcsFuseCsiDriverConfig>,
    /// The status of the Backup for GKE Agent addon. It is disabled by default. Set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "gkeBackupAgentConfigs")]
    pub r#gke_backup_agent_configs: Vec<super::super::types::container::GetClusterAddonsConfigGkeBackupAgentConfig>,
    /// The status of the Horizontal Pod Autoscaling addon, which increases or decreases the number of replica pods a replication controller has based on the resource usage of the existing pods. It ensures that a Heapster pod is running in the cluster, which is also used by the Cloud Monitoring service. It is enabled by default; set disabled = true to disable.
    #[builder(into)]
    #[serde(rename = "horizontalPodAutoscalings")]
    pub r#horizontal_pod_autoscalings: Vec<super::super::types::container::GetClusterAddonsConfigHorizontalPodAutoscaling>,
    /// The status of the HTTP (L7) load balancing controller addon, which makes it easy to set up HTTP load balancers for services in a cluster. It is enabled by default; set disabled = true to disable.
    #[builder(into)]
    #[serde(rename = "httpLoadBalancings")]
    pub r#http_load_balancings: Vec<super::super::types::container::GetClusterAddonsConfigHttpLoadBalancing>,
    /// The status of the Istio addon.
    #[builder(into)]
    #[serde(rename = "istioConfigs")]
    pub r#istio_configs: Vec<super::super::types::container::GetClusterAddonsConfigIstioConfig>,
    /// Configuration for the KALM addon, which manages the lifecycle of k8s. It is disabled by default; Set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "kalmConfigs")]
    pub r#kalm_configs: Vec<super::super::types::container::GetClusterAddonsConfigKalmConfig>,
    /// Whether we should enable the network policy addon for the master. This must be enabled in order to enable network policy for the nodes. To enable this, you must also define a network_policy block, otherwise nothing will happen. It can only be disabled if the nodes already do not have network policies enabled. Defaults to disabled; set disabled = false to enable.
    #[builder(into)]
    #[serde(rename = "networkPolicyConfigs")]
    pub r#network_policy_configs: Vec<super::super::types::container::GetClusterAddonsConfigNetworkPolicyConfig>,
    /// The status of the Parallelstore CSI driver addon, which allows the usage of Parallelstore instances as volumes. Defaults to disabled; set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "parallelstoreCsiDriverConfigs")]
    pub r#parallelstore_csi_driver_configs: Vec<super::super::types::container::GetClusterAddonsConfigParallelstoreCsiDriverConfig>,
    /// The status of the Ray Operator addon, which enabled management of Ray AI/ML jobs on GKE. Defaults to disabled; set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "rayOperatorConfigs")]
    pub r#ray_operator_configs: Vec<super::super::types::container::GetClusterAddonsConfigRayOperatorConfig>,
    /// The status of the Stateful HA addon, which provides automatic configurable failover for stateful applications. Defaults to disabled; set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "statefulHaConfigs")]
    pub r#stateful_ha_configs: Vec<super::super::types::container::GetClusterAddonsConfigStatefulHaConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterAddonsConfig {
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
                    "cloudrunConfigs",
                    &self.r#cloudrun_configs,
                ),
                to_pulumi_object_field(
                    "configConnectorConfigs",
                    &self.r#config_connector_configs,
                ),
                to_pulumi_object_field(
                    "dnsCacheConfigs",
                    &self.r#dns_cache_configs,
                ),
                to_pulumi_object_field(
                    "gcePersistentDiskCsiDriverConfigs",
                    &self.r#gce_persistent_disk_csi_driver_configs,
                ),
                to_pulumi_object_field(
                    "gcpFilestoreCsiDriverConfigs",
                    &self.r#gcp_filestore_csi_driver_configs,
                ),
                to_pulumi_object_field(
                    "gcsFuseCsiDriverConfigs",
                    &self.r#gcs_fuse_csi_driver_configs,
                ),
                to_pulumi_object_field(
                    "gkeBackupAgentConfigs",
                    &self.r#gke_backup_agent_configs,
                ),
                to_pulumi_object_field(
                    "horizontalPodAutoscalings",
                    &self.r#horizontal_pod_autoscalings,
                ),
                to_pulumi_object_field(
                    "httpLoadBalancings",
                    &self.r#http_load_balancings,
                ),
                to_pulumi_object_field(
                    "istioConfigs",
                    &self.r#istio_configs,
                ),
                to_pulumi_object_field(
                    "kalmConfigs",
                    &self.r#kalm_configs,
                ),
                to_pulumi_object_field(
                    "networkPolicyConfigs",
                    &self.r#network_policy_configs,
                ),
                to_pulumi_object_field(
                    "parallelstoreCsiDriverConfigs",
                    &self.r#parallelstore_csi_driver_configs,
                ),
                to_pulumi_object_field(
                    "rayOperatorConfigs",
                    &self.r#ray_operator_configs,
                ),
                to_pulumi_object_field(
                    "statefulHaConfigs",
                    &self.r#stateful_ha_configs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterAddonsConfig {
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
                    r#cloudrun_configs: {
                        let field_value = match fields_map.get("cloudrunConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudrunConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#config_connector_configs: {
                        let field_value = match fields_map.get("configConnectorConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'configConnectorConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_cache_configs: {
                        let field_value = match fields_map.get("dnsCacheConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'dnsCacheConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gce_persistent_disk_csi_driver_configs: {
                        let field_value = match fields_map.get("gcePersistentDiskCsiDriverConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcePersistentDiskCsiDriverConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcp_filestore_csi_driver_configs: {
                        let field_value = match fields_map.get("gcpFilestoreCsiDriverConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcpFilestoreCsiDriverConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcs_fuse_csi_driver_configs: {
                        let field_value = match fields_map.get("gcsFuseCsiDriverConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcsFuseCsiDriverConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gke_backup_agent_configs: {
                        let field_value = match fields_map.get("gkeBackupAgentConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'gkeBackupAgentConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#horizontal_pod_autoscalings: {
                        let field_value = match fields_map.get("horizontalPodAutoscalings") {
                            Some(value) => value,
                            None => bail!("Missing field 'horizontalPodAutoscalings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_load_balancings: {
                        let field_value = match fields_map.get("httpLoadBalancings") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpLoadBalancings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#istio_configs: {
                        let field_value = match fields_map.get("istioConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'istioConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kalm_configs: {
                        let field_value = match fields_map.get("kalmConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'kalmConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_policy_configs: {
                        let field_value = match fields_map.get("networkPolicyConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkPolicyConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parallelstore_csi_driver_configs: {
                        let field_value = match fields_map.get("parallelstoreCsiDriverConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'parallelstoreCsiDriverConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ray_operator_configs: {
                        let field_value = match fields_map.get("rayOperatorConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'rayOperatorConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateful_ha_configs: {
                        let field_value = match fields_map.get("statefulHaConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'statefulHaConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
