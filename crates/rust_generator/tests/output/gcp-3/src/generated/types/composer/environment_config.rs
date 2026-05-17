#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EnvironmentConfig {
    /// The URI of the Apache Airflow Web UI hosted within this
    /// environment.
    #[builder(into)]
    #[serde(rename = "airflowUri")]
    pub r#airflow_uri: Option<String>,
    /// The Cloud Storage prefix of the DAGs for this environment.
    /// Although Cloud Storage objects reside in a flat namespace, a
    /// hierarchical file tree can be simulated using '/'-delimited
    /// object name prefixes. DAG objects for this environment
    /// reside in a simulated directory with this prefix.
    #[builder(into)]
    #[serde(rename = "dagGcsPrefix")]
    pub r#dag_gcs_prefix: Option<String>,
    /// The configuration setting for Airflow data retention mechanism. This field is supported for Cloud Composer environments in versions composer-2.0.32-airflow-2.1.4. or newer
    #[builder(into)]
    #[serde(rename = "dataRetentionConfig")]
    pub r#data_retention_config: Option<Box<super::super::types::composer::EnvironmentConfigDataRetentionConfig>>,
    /// The configuration of Cloud SQL instance that is used by the Apache Airflow software. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    #[serde(rename = "databaseConfig")]
    pub r#database_config: Option<Box<super::super::types::composer::EnvironmentConfigDatabaseConfig>>,
    /// Optional. If true, builds performed during operations that install Python packages have only private connectivity to Google services. If false, the builds also have access to the internet.
    #[builder(into)]
    #[serde(rename = "enablePrivateBuildsOnly")]
    pub r#enable_private_builds_only: Option<bool>,
    /// Optional. If true, a private Composer environment will be created.
    #[builder(into)]
    #[serde(rename = "enablePrivateEnvironment")]
    pub r#enable_private_environment: Option<bool>,
    /// The encryption options for the Composer environment and its dependencies.
    #[builder(into)]
    #[serde(rename = "encryptionConfig")]
    pub r#encryption_config: Option<Box<super::super::types::composer::EnvironmentConfigEncryptionConfig>>,
    /// The size of the Cloud Composer environment. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[builder(into)]
    #[serde(rename = "environmentSize")]
    pub r#environment_size: Option<String>,
    /// The Kubernetes Engine cluster used to run this environment.
    #[builder(into)]
    #[serde(rename = "gkeCluster")]
    pub r#gke_cluster: Option<String>,
    /// The configuration for Cloud Composer maintenance window.
    #[builder(into)]
    #[serde(rename = "maintenanceWindow")]
    pub r#maintenance_window: Option<Box<super::super::types::composer::EnvironmentConfigMaintenanceWindow>>,
    /// Configuration options for the master authorized networks feature. Enabled master authorized networks will disallow all external traffic to access Kubernetes master through HTTPS except traffic from the given CIDR blocks, Google Compute Engine Public IPs and Google Prod IPs.
    #[builder(into)]
    #[serde(rename = "masterAuthorizedNetworksConfig")]
    pub r#master_authorized_networks_config: Option<Box<super::super::types::composer::EnvironmentConfigMasterAuthorizedNetworksConfig>>,
    /// The configuration used for the Kubernetes Engine cluster.
    #[builder(into)]
    #[serde(rename = "nodeConfig")]
    pub r#node_config: Option<Box<super::super::types::composer::EnvironmentConfigNodeConfig>>,
    /// The number of nodes in the Kubernetes Engine cluster that will be used to run this environment. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Option<i32>,
    /// The configuration used for the Private IP Cloud Composer environment.
    #[builder(into)]
    #[serde(rename = "privateEnvironmentConfig")]
    pub r#private_environment_config: Option<Box<super::super::types::composer::EnvironmentConfigPrivateEnvironmentConfig>>,
    /// The recovery configuration settings for the Cloud Composer environment
    #[builder(into)]
    #[serde(rename = "recoveryConfig")]
    pub r#recovery_config: Option<Box<super::super::types::composer::EnvironmentConfigRecoveryConfig>>,
    /// Whether high resilience is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.1.15-airflow-*.*.* and newer.
    #[builder(into)]
    #[serde(rename = "resilienceMode")]
    pub r#resilience_mode: Option<String>,
    /// The configuration settings for software inside the environment.
    #[builder(into)]
    #[serde(rename = "softwareConfig")]
    pub r#software_config: Option<Box<super::super::types::composer::EnvironmentConfigSoftwareConfig>>,
    /// The configuration settings for the Airflow web server App Engine instance. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    #[serde(rename = "webServerConfig")]
    pub r#web_server_config: Option<Box<super::super::types::composer::EnvironmentConfigWebServerConfig>>,
    /// Network-level access control policy for the Airflow web server.
    #[builder(into)]
    #[serde(rename = "webServerNetworkAccessControl")]
    pub r#web_server_network_access_control: Option<Box<super::super::types::composer::EnvironmentConfigWebServerNetworkAccessControl>>,
    /// The workloads configuration settings for the GKE cluster associated with the Cloud Composer environment. Supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[builder(into)]
    #[serde(rename = "workloadsConfig")]
    pub r#workloads_config: Option<Box<super::super::types::composer::EnvironmentConfigWorkloadsConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EnvironmentConfig {
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
                    "airflow_uri",
                    &self.r#airflow_uri,
                ),
                to_pulumi_object_field(
                    "dag_gcs_prefix",
                    &self.r#dag_gcs_prefix,
                ),
                to_pulumi_object_field(
                    "data_retention_config",
                    &self.r#data_retention_config,
                ),
                to_pulumi_object_field(
                    "database_config",
                    &self.r#database_config,
                ),
                to_pulumi_object_field(
                    "enable_private_builds_only",
                    &self.r#enable_private_builds_only,
                ),
                to_pulumi_object_field(
                    "enable_private_environment",
                    &self.r#enable_private_environment,
                ),
                to_pulumi_object_field(
                    "encryption_config",
                    &self.r#encryption_config,
                ),
                to_pulumi_object_field(
                    "environment_size",
                    &self.r#environment_size,
                ),
                to_pulumi_object_field(
                    "gke_cluster",
                    &self.r#gke_cluster,
                ),
                to_pulumi_object_field(
                    "maintenance_window",
                    &self.r#maintenance_window,
                ),
                to_pulumi_object_field(
                    "master_authorized_networks_config",
                    &self.r#master_authorized_networks_config,
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
                    "private_environment_config",
                    &self.r#private_environment_config,
                ),
                to_pulumi_object_field(
                    "recovery_config",
                    &self.r#recovery_config,
                ),
                to_pulumi_object_field(
                    "resilience_mode",
                    &self.r#resilience_mode,
                ),
                to_pulumi_object_field(
                    "software_config",
                    &self.r#software_config,
                ),
                to_pulumi_object_field(
                    "web_server_config",
                    &self.r#web_server_config,
                ),
                to_pulumi_object_field(
                    "web_server_network_access_control",
                    &self.r#web_server_network_access_control,
                ),
                to_pulumi_object_field(
                    "workloads_config",
                    &self.r#workloads_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("airflow_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'airflow_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dag_gcs_prefix: {
                        let field_value = match fields_map.get("dag_gcs_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'dag_gcs_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_retention_config: {
                        let field_value = match fields_map.get("data_retention_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_retention_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_config: {
                        let field_value = match fields_map.get("database_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_private_builds_only: {
                        let field_value = match fields_map.get("enable_private_builds_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_private_builds_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_private_environment: {
                        let field_value = match fields_map.get("enable_private_environment") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_private_environment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#environment_size: {
                        let field_value = match fields_map.get("environment_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'environment_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gke_cluster: {
                        let field_value = match fields_map.get("gke_cluster") {
                            Some(value) => value,
                            None => bail!("Missing field 'gke_cluster' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maintenance_window: {
                        let field_value = match fields_map.get("maintenance_window") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenance_window' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#master_authorized_networks_config: {
                        let field_value = match fields_map.get("master_authorized_networks_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'master_authorized_networks_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#private_environment_config: {
                        let field_value = match fields_map.get("private_environment_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_environment_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recovery_config: {
                        let field_value = match fields_map.get("recovery_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'recovery_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resilience_mode: {
                        let field_value = match fields_map.get("resilience_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'resilience_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#web_server_config: {
                        let field_value = match fields_map.get("web_server_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'web_server_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_server_network_access_control: {
                        let field_value = match fields_map.get("web_server_network_access_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'web_server_network_access_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workloads_config: {
                        let field_value = match fields_map.get("workloads_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'workloads_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
