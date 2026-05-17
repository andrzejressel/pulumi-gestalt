#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkflowTemplatePlacementManagedClusterConfig {
    /// Autoscaling config for the policy associated with the cluster. Cluster does not autoscale if this field is unset.
    #[builder(into)]
    #[serde(rename = "autoscalingConfig")]
    pub r#autoscaling_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigAutoscalingConfig>>,
    /// Encryption settings for the cluster.
    #[builder(into)]
    #[serde(rename = "encryptionConfig")]
    pub r#encryption_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigEncryptionConfig>>,
    /// Port/endpoint configuration for this cluster
    #[builder(into)]
    #[serde(rename = "endpointConfig")]
    pub r#endpoint_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigEndpointConfig>>,
    /// The shared Compute Engine config settings for all instances in a cluster.
    #[builder(into)]
    #[serde(rename = "gceClusterConfig")]
    pub r#gce_cluster_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigGceClusterConfig>>,
    /// The Kubernetes Engine config for Dataproc clusters deployed to Kubernetes. Setting this is considered mutually exclusive with Compute Engine-based options such as `gce_cluster_config`, `master_config`, `worker_config`, `secondary_worker_config`, and `autoscaling_config`.
    #[builder(into)]
    #[serde(rename = "gkeClusterConfig")]
    pub r#gke_cluster_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigGkeClusterConfig>>,
    /// Commands to execute on each node after config is completed. By default, executables are run on master and all worker nodes. You can test a node's `role` metadata to run an executable on a master or worker node, as shown below using `curl` (you can also use `wget`): ROLE=$(curl -H Metadata-Flavor:Google http://metadata/computeMetadata/v1/instance/attributes/dataproc-role) if ; then ... master specific actions ... else ... worker specific actions ... fi
    #[builder(into)]
    #[serde(rename = "initializationActions")]
    pub r#initialization_actions: Option<Vec<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigInitializationAction>>,
    /// Lifecycle setting for the cluster.
    #[builder(into)]
    #[serde(rename = "lifecycleConfig")]
    pub r#lifecycle_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigLifecycleConfig>>,
    /// The Compute Engine config settings for additional worker instances in a cluster.
    #[builder(into)]
    #[serde(rename = "masterConfig")]
    pub r#master_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigMasterConfig>>,
    /// Metastore configuration.
    #[builder(into)]
    #[serde(rename = "metastoreConfig")]
    pub r#metastore_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigMetastoreConfig>>,
    /// The Compute Engine config settings for additional worker instances in a cluster.
    #[builder(into)]
    #[serde(rename = "secondaryWorkerConfig")]
    pub r#secondary_worker_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigSecondaryWorkerConfig>>,
    /// Security settings for the cluster.
    #[builder(into)]
    #[serde(rename = "securityConfig")]
    pub r#security_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigSecurityConfig>>,
    /// The config settings for software inside the cluster.
    #[builder(into)]
    #[serde(rename = "softwareConfig")]
    pub r#software_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigSoftwareConfig>>,
    /// A Cloud Storage bucket used to stage job dependencies, config files, and job driver console output. If you do not specify a staging bucket, Cloud Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's staging bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket (see [Dataproc staging and temp buckets](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/staging-bucket)).
    #[builder(into)]
    #[serde(rename = "stagingBucket")]
    pub r#staging_bucket: Option<String>,
    /// A Cloud Storage bucket used to store ephemeral cluster and jobs data, such as Spark and MapReduce history files. If you do not specify a temp bucket, Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's temp bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket. The default bucket has a TTL of 90 days, but you can use any TTL (or none) if you specify a bucket.
    #[builder(into)]
    #[serde(rename = "tempBucket")]
    pub r#temp_bucket: Option<String>,
    /// The Compute Engine config settings for additional worker instances in a cluster.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "workerConfig")]
    pub r#worker_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigWorkerConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkflowTemplatePlacementManagedClusterConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

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
                "gke_cluster_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gke_cluster_config,
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
                "secondary_worker_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secondary_worker_config,
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
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkflowTemplatePlacementManagedClusterConfig {
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
                    r#gke_cluster_config: {
                        let field_value = match fields_map.get("gke_cluster_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'gke_cluster_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#secondary_worker_config: {
                        let field_value = match fields_map.get("secondary_worker_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary_worker_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
