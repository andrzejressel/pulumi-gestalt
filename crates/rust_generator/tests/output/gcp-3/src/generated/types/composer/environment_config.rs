#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EnvironmentConfig {
    /// The URI of the Apache Airflow Web UI hosted within this
    /// environment.
    #[builder(into)]
    pub r#airflow_uri: Option<String>,
    /// The Cloud Storage prefix of the DAGs for this environment.
    /// Although Cloud Storage objects reside in a flat namespace, a
    /// hierarchical file tree can be simulated using '/'-delimited
    /// object name prefixes. DAG objects for this environment
    /// reside in a simulated directory with this prefix.
    #[builder(into)]
    pub r#dag_gcs_prefix: Option<String>,
    /// The configuration setting for Airflow data retention mechanism. This field is supported for Cloud Composer environments in versions composer-2.0.32-airflow-2.1.4. or newer
    #[builder(into)]
    pub r#data_retention_config: Option<Box<super::super::types::composer::EnvironmentConfigDataRetentionConfig>>,
    /// The configuration of Cloud SQL instance that is used by the Apache Airflow software. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    pub r#database_config: Option<Box<super::super::types::composer::EnvironmentConfigDatabaseConfig>>,
    /// Optional. If true, builds performed during operations that install Python packages have only private connectivity to Google services. If false, the builds also have access to the internet.
    #[builder(into)]
    pub r#enable_private_builds_only: Option<bool>,
    /// Optional. If true, a private Composer environment will be created.
    #[builder(into)]
    pub r#enable_private_environment: Option<bool>,
    /// The encryption options for the Composer environment and its dependencies.
    #[builder(into)]
    pub r#encryption_config: Option<Box<super::super::types::composer::EnvironmentConfigEncryptionConfig>>,
    /// The size of the Cloud Composer environment. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[builder(into)]
    pub r#environment_size: Option<String>,
    /// The Kubernetes Engine cluster used to run this environment.
    #[builder(into)]
    pub r#gke_cluster: Option<String>,
    /// The configuration for Cloud Composer maintenance window.
    #[builder(into)]
    pub r#maintenance_window: Option<Box<super::super::types::composer::EnvironmentConfigMaintenanceWindow>>,
    /// Configuration options for the master authorized networks feature. Enabled master authorized networks will disallow all external traffic to access Kubernetes master through HTTPS except traffic from the given CIDR blocks, Google Compute Engine Public IPs and Google Prod IPs.
    #[builder(into)]
    pub r#master_authorized_networks_config: Option<Box<super::super::types::composer::EnvironmentConfigMasterAuthorizedNetworksConfig>>,
    /// The configuration used for the Kubernetes Engine cluster.
    #[builder(into)]
    pub r#node_config: Option<Box<super::super::types::composer::EnvironmentConfigNodeConfig>>,
    /// The number of nodes in the Kubernetes Engine cluster that will be used to run this environment. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    pub r#node_count: Option<i32>,
    /// The configuration used for the Private IP Cloud Composer environment.
    #[builder(into)]
    pub r#private_environment_config: Option<Box<super::super::types::composer::EnvironmentConfigPrivateEnvironmentConfig>>,
    /// The recovery configuration settings for the Cloud Composer environment
    #[builder(into)]
    pub r#recovery_config: Option<Box<super::super::types::composer::EnvironmentConfigRecoveryConfig>>,
    /// Whether high resilience is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.1.15-airflow-*.*.* and newer.
    #[builder(into)]
    pub r#resilience_mode: Option<String>,
    /// The configuration settings for software inside the environment.
    #[builder(into)]
    pub r#software_config: Option<Box<super::super::types::composer::EnvironmentConfigSoftwareConfig>>,
    /// The configuration settings for the Airflow web server App Engine instance. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    pub r#web_server_config: Option<Box<super::super::types::composer::EnvironmentConfigWebServerConfig>>,
    /// Network-level access control policy for the Airflow web server.
    #[builder(into)]
    pub r#web_server_network_access_control: Option<Box<super::super::types::composer::EnvironmentConfigWebServerNetworkAccessControl>>,
    /// The workloads configuration settings for the GKE cluster associated with the Cloud Composer environment. Supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[builder(into)]
    pub r#workloads_config: Option<Box<super::super::types::composer::EnvironmentConfigWorkloadsConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EnvironmentConfig {
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
                    "airflowUri",
                    &self.r#airflow_uri,
                ),
                to_pulumi_object_field(
                    "dagGcsPrefix",
                    &self.r#dag_gcs_prefix,
                ),
                to_pulumi_object_field(
                    "dataRetentionConfig",
                    &self.r#data_retention_config,
                ),
                to_pulumi_object_field(
                    "databaseConfig",
                    &self.r#database_config,
                ),
                to_pulumi_object_field(
                    "enablePrivateBuildsOnly",
                    &self.r#enable_private_builds_only,
                ),
                to_pulumi_object_field(
                    "enablePrivateEnvironment",
                    &self.r#enable_private_environment,
                ),
                to_pulumi_object_field(
                    "encryptionConfig",
                    &self.r#encryption_config,
                ),
                to_pulumi_object_field(
                    "environmentSize",
                    &self.r#environment_size,
                ),
                to_pulumi_object_field(
                    "gkeCluster",
                    &self.r#gke_cluster,
                ),
                to_pulumi_object_field(
                    "maintenanceWindow",
                    &self.r#maintenance_window,
                ),
                to_pulumi_object_field(
                    "masterAuthorizedNetworksConfig",
                    &self.r#master_authorized_networks_config,
                ),
                to_pulumi_object_field(
                    "nodeConfig",
                    &self.r#node_config,
                ),
                to_pulumi_object_field(
                    "nodeCount",
                    &self.r#node_count,
                ),
                to_pulumi_object_field(
                    "privateEnvironmentConfig",
                    &self.r#private_environment_config,
                ),
                to_pulumi_object_field(
                    "recoveryConfig",
                    &self.r#recovery_config,
                ),
                to_pulumi_object_field(
                    "resilienceMode",
                    &self.r#resilience_mode,
                ),
                to_pulumi_object_field(
                    "softwareConfig",
                    &self.r#software_config,
                ),
                to_pulumi_object_field(
                    "webServerConfig",
                    &self.r#web_server_config,
                ),
                to_pulumi_object_field(
                    "webServerNetworkAccessControl",
                    &self.r#web_server_network_access_control,
                ),
                to_pulumi_object_field(
                    "workloadsConfig",
                    &self.r#workloads_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EnvironmentConfig {
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
                    r#airflow_uri: {
                        let field_value = match fields_map.get("airflowUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'airflowUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dag_gcs_prefix: {
                        let field_value = match fields_map.get("dagGcsPrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'dagGcsPrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_retention_config: {
                        let field_value = match fields_map.get("dataRetentionConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataRetentionConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_config: {
                        let field_value = match fields_map.get("databaseConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'databaseConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_private_builds_only: {
                        let field_value = match fields_map.get("enablePrivateBuildsOnly") {
                            Some(value) => value,
                            None => bail!("Missing field 'enablePrivateBuildsOnly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_private_environment: {
                        let field_value = match fields_map.get("enablePrivateEnvironment") {
                            Some(value) => value,
                            None => bail!("Missing field 'enablePrivateEnvironment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_config: {
                        let field_value = match fields_map.get("encryptionConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryptionConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#environment_size: {
                        let field_value = match fields_map.get("environmentSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'environmentSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gke_cluster: {
                        let field_value = match fields_map.get("gkeCluster") {
                            Some(value) => value,
                            None => bail!("Missing field 'gkeCluster' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maintenance_window: {
                        let field_value = match fields_map.get("maintenanceWindow") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenanceWindow' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#master_authorized_networks_config: {
                        let field_value = match fields_map.get("masterAuthorizedNetworksConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'masterAuthorizedNetworksConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_config: {
                        let field_value = match fields_map.get("nodeConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodeConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_count: {
                        let field_value = match fields_map.get("nodeCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodeCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_environment_config: {
                        let field_value = match fields_map.get("privateEnvironmentConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateEnvironmentConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recovery_config: {
                        let field_value = match fields_map.get("recoveryConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'recoveryConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resilience_mode: {
                        let field_value = match fields_map.get("resilienceMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'resilienceMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#software_config: {
                        let field_value = match fields_map.get("softwareConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'softwareConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_server_config: {
                        let field_value = match fields_map.get("webServerConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'webServerConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_server_network_access_control: {
                        let field_value = match fields_map.get("webServerNetworkAccessControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'webServerNetworkAccessControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workloads_config: {
                        let field_value = match fields_map.get("workloadsConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'workloadsConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
