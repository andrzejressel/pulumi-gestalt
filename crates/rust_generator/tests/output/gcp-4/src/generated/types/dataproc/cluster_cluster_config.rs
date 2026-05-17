#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterConfig {
    /// The autoscaling policy config associated with the cluster.
    /// Note that once set, if `autoscaling_config` is the only field set in `cluster_config`, it can
    /// only be removed by setting `policy_uri = ""`, rather than removing the whole block.
    /// Structure defined below.
    #[builder(into)]
    #[serde(rename = "autoscalingConfig")]
    pub r#autoscaling_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigAutoscalingConfig>>,
    /// A Dataproc NodeGroup resource is a group of Dataproc cluster nodes that execute an assigned role. 
    /// Structure defined below.
    #[builder(into)]
    #[serde(rename = "auxiliaryNodeGroups")]
    pub r#auxiliary_node_groups: Option<Vec<super::super::types::dataproc::ClusterClusterConfigAuxiliaryNodeGroup>>,
    /// The name of the cloud storage bucket ultimately used to house the staging data
    /// for the cluster. If `staging_bucket` is specified, it will contain this value, otherwise
    /// it will be the auto generated name.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Option<String>,
    /// The Compute Engine accelerator (GPU) configuration for these instances. Can be specified multiple times.
    /// Structure defined below.
    #[builder(into)]
    #[serde(rename = "dataprocMetricConfig")]
    pub r#dataproc_metric_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigDataprocMetricConfig>>,
    /// The Customer managed encryption keys settings for the cluster.
    /// Structure defined below.
    #[builder(into)]
    #[serde(rename = "encryptionConfig")]
    pub r#encryption_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigEncryptionConfig>>,
    /// The config settings for port access on the cluster.
    /// Structure defined below.
    #[builder(into)]
    #[serde(rename = "endpointConfig")]
    pub r#endpoint_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigEndpointConfig>>,
    /// Common config settings for resources of Google Compute Engine cluster
    /// instances, applicable to all instances in the cluster. Structure defined below.
    #[builder(into)]
    #[serde(rename = "gceClusterConfig")]
    pub r#gce_cluster_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigGceClusterConfig>>,
    /// Commands to execute on each node after config is completed.
    /// You can specify multiple versions of these. Structure defined below.
    #[builder(into)]
    #[serde(rename = "initializationActions")]
    pub r#initialization_actions: Option<Vec<super::super::types::dataproc::ClusterClusterConfigInitializationAction>>,
    /// The settings for auto deletion cluster schedule.
    /// Structure defined below.
    #[builder(into)]
    #[serde(rename = "lifecycleConfig")]
    pub r#lifecycle_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigLifecycleConfig>>,
    /// The Google Compute Engine config settings for the master instances
    /// in a cluster. Structure defined below.
    #[builder(into)]
    #[serde(rename = "masterConfig")]
    pub r#master_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigMasterConfig>>,
    /// The config setting for metastore service with the cluster.
    /// Structure defined below.
    /// - - -
    #[builder(into)]
    #[serde(rename = "metastoreConfig")]
    pub r#metastore_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigMetastoreConfig>>,
    /// The Google Compute Engine config settings for the additional
    /// instances in a cluster. Structure defined below.
    /// * **NOTE** : `preemptible_worker_config` is
    /// an alias for the api's [secondaryWorkerConfig](https://cloud.google.com/dataproc/docs/reference/rest/v1/ClusterConfig#InstanceGroupConfig). The name doesn't necessarily mean it is preemptible and is named as
    /// such for legacy/compatibility reasons.
    #[builder(into)]
    #[serde(rename = "preemptibleWorkerConfig")]
    pub r#preemptible_worker_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigPreemptibleWorkerConfig>>,
    /// Security related configuration. Structure defined below.
    #[builder(into)]
    #[serde(rename = "securityConfig")]
    pub r#security_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigSecurityConfig>>,
    /// The config settings for software inside the cluster.
    /// Structure defined below.
    #[builder(into)]
    #[serde(rename = "softwareConfig")]
    pub r#software_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigSoftwareConfig>>,
    /// The Cloud Storage staging bucket used to stage files,
    /// such as Hadoop jars, between client machines and the cluster.
    /// Note: If you don't explicitly specify a `staging_bucket`
    /// then GCP will auto create / assign one for you. However, you are not guaranteed
    /// an auto generated bucket which is solely dedicated to your cluster; it may be shared
    /// with other clusters in the same region/zone also choosing to use the auto generation
    /// option.
    #[builder(into)]
    #[serde(rename = "stagingBucket")]
    pub r#staging_bucket: Option<String>,
    /// The Cloud Storage temp bucket used to store ephemeral cluster
    /// and jobs data, such as Spark and MapReduce history files.
    /// Note: If you don't explicitly specify a `temp_bucket` then GCP will auto create / assign one for you.
    #[builder(into)]
    #[serde(rename = "tempBucket")]
    pub r#temp_bucket: Option<String>,
    /// The Google Compute Engine config settings for the worker instances
    /// in a cluster. Structure defined below.
    #[builder(into)]
    #[serde(rename = "workerConfig")]
    pub r#worker_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigWorkerConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterClusterConfig {
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
                "autoscaling_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#autoscaling_config,
                )
                .await,
            );
            map.insert(
                "auxiliary_node_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auxiliary_node_groups,
                )
                .await,
            );
            map.insert(
                "bucket".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bucket,
                )
                .await,
            );
            map.insert(
                "dataproc_metric_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dataproc_metric_config,
                )
                .await,
            );
            map.insert(
                "encryption_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption_config,
                )
                .await,
            );
            map.insert(
                "endpoint_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#endpoint_config,
                )
                .await,
            );
            map.insert(
                "gce_cluster_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gce_cluster_config,
                )
                .await,
            );
            map.insert(
                "initialization_actions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#initialization_actions,
                )
                .await,
            );
            map.insert(
                "lifecycle_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lifecycle_config,
                )
                .await,
            );
            map.insert(
                "master_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#master_config,
                )
                .await,
            );
            map.insert(
                "metastore_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metastore_config,
                )
                .await,
            );
            map.insert(
                "preemptible_worker_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preemptible_worker_config,
                )
                .await,
            );
            map.insert(
                "security_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_config,
                )
                .await,
            );
            map.insert(
                "software_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#software_config,
                )
                .await,
            );
            map.insert(
                "staging_bucket".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#staging_bucket,
                )
                .await,
            );
            map.insert(
                "temp_bucket".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#temp_bucket,
                )
                .await,
            );
            map.insert(
                "worker_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#worker_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterClusterConfig {
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
                    r#autoscaling_config: {
                        let field_value = match fields_map.get("autoscaling_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoscaling_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auxiliary_node_groups: {
                        let field_value = match fields_map.get("auxiliary_node_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'auxiliary_node_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket: {
                        let field_value = match fields_map.get("bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dataproc_metric_config: {
                        let field_value = match fields_map.get("dataproc_metric_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataproc_metric_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_config: {
                        let field_value = match fields_map.get("encryption_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#endpoint_config: {
                        let field_value = match fields_map.get("endpoint_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gce_cluster_config: {
                        let field_value = match fields_map.get("gce_cluster_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'gce_cluster_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#initialization_actions: {
                        let field_value = match fields_map.get("initialization_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'initialization_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifecycle_config: {
                        let field_value = match fields_map.get("lifecycle_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycle_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#master_config: {
                        let field_value = match fields_map.get("master_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'master_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metastore_config: {
                        let field_value = match fields_map.get("metastore_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'metastore_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preemptible_worker_config: {
                        let field_value = match fields_map.get("preemptible_worker_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'preemptible_worker_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_config: {
                        let field_value = match fields_map.get("security_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#software_config: {
                        let field_value = match fields_map.get("software_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'software_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#staging_bucket: {
                        let field_value = match fields_map.get("staging_bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'staging_bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#temp_bucket: {
                        let field_value = match fields_map.get("temp_bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'temp_bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#worker_config: {
                        let field_value = match fields_map.get("worker_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'worker_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
