#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEnvironmentConfig {
    /// The URI of the Apache Airflow Web UI hosted within the
    /// environment.
    #[builder(into)]
    pub r#airflow_uri: String,
    /// The Cloud Storage prefix of the DAGs for the environment.
    #[builder(into)]
    pub r#dag_gcs_prefix: String,
    /// The configuration setting for Airflow data retention mechanism. This field is supported for Cloud Composer environments in versions composer-2.0.32-airflow-2.1.4. or newer
    #[builder(into)]
    pub r#data_retention_configs: Vec<super::super::types::composer::GetEnvironmentConfigDataRetentionConfig>,
    /// The configuration of Cloud SQL instance that is used by the Apache Airflow software. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    pub r#database_configs: Vec<super::super::types::composer::GetEnvironmentConfigDatabaseConfig>,
    /// Optional. If true, builds performed during operations that install Python packages have only private connectivity to Google services. If false, the builds also have access to the internet.
    #[builder(into)]
    pub r#enable_private_builds_only: bool,
    /// Optional. If true, a private Composer environment will be created.
    #[builder(into)]
    pub r#enable_private_environment: bool,
    /// The encryption options for the Composer environment and its dependencies.
    #[builder(into)]
    pub r#encryption_configs: Vec<super::super::types::composer::GetEnvironmentConfigEncryptionConfig>,
    /// The size of the Cloud Composer environment. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[builder(into)]
    pub r#environment_size: String,
    /// The Kubernetes Engine cluster used to run the environment.
    #[builder(into)]
    pub r#gke_cluster: String,
    /// The configuration for Cloud Composer maintenance window.
    #[builder(into)]
    pub r#maintenance_windows: Vec<super::super::types::composer::GetEnvironmentConfigMaintenanceWindow>,
    /// Configuration options for the master authorized networks feature. Enabled master authorized networks will disallow all external traffic to access Kubernetes master through HTTPS except traffic from the given CIDR blocks, Google Compute Engine Public IPs and Google Prod IPs.
    #[builder(into)]
    pub r#master_authorized_networks_configs: Vec<super::super::types::composer::GetEnvironmentConfigMasterAuthorizedNetworksConfig>,
    /// The configuration used for the Kubernetes Engine cluster.
    #[builder(into)]
    pub r#node_configs: Vec<super::super::types::composer::GetEnvironmentConfigNodeConfig>,
    /// The number of nodes in the Kubernetes Engine cluster that will be used to run this environment. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    pub r#node_count: i32,
    /// The configuration used for the Private IP Cloud Composer environment.
    #[builder(into)]
    pub r#private_environment_configs: Vec<super::super::types::composer::GetEnvironmentConfigPrivateEnvironmentConfig>,
    /// The recovery configuration settings for the Cloud Composer environment
    #[builder(into)]
    pub r#recovery_configs: Vec<super::super::types::composer::GetEnvironmentConfigRecoveryConfig>,
    /// Whether high resilience is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.1.15-airflow-*.*.* and newer.
    #[builder(into)]
    pub r#resilience_mode: String,
    /// The configuration settings for software inside the environment.
    #[builder(into)]
    pub r#software_configs: Vec<super::super::types::composer::GetEnvironmentConfigSoftwareConfig>,
    /// The configuration settings for the Airflow web server App Engine instance. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    pub r#web_server_configs: Vec<super::super::types::composer::GetEnvironmentConfigWebServerConfig>,
    /// Network-level access control policy for the Airflow web server.
    #[builder(into)]
    pub r#web_server_network_access_controls: Vec<super::super::types::composer::GetEnvironmentConfigWebServerNetworkAccessControl>,
    /// The workloads configuration settings for the GKE cluster associated with the Cloud Composer environment. Supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[builder(into)]
    pub r#workloads_configs: Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetEnvironmentConfig {
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
                    "dataRetentionConfigs",
                    &self.r#data_retention_configs,
                ),
                to_pulumi_object_field(
                    "databaseConfigs",
                    &self.r#database_configs,
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
                    "encryptionConfigs",
                    &self.r#encryption_configs,
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
                    "maintenanceWindows",
                    &self.r#maintenance_windows,
                ),
                to_pulumi_object_field(
                    "masterAuthorizedNetworksConfigs",
                    &self.r#master_authorized_networks_configs,
                ),
                to_pulumi_object_field(
                    "nodeConfigs",
                    &self.r#node_configs,
                ),
                to_pulumi_object_field(
                    "nodeCount",
                    &self.r#node_count,
                ),
                to_pulumi_object_field(
                    "privateEnvironmentConfigs",
                    &self.r#private_environment_configs,
                ),
                to_pulumi_object_field(
                    "recoveryConfigs",
                    &self.r#recovery_configs,
                ),
                to_pulumi_object_field(
                    "resilienceMode",
                    &self.r#resilience_mode,
                ),
                to_pulumi_object_field(
                    "softwareConfigs",
                    &self.r#software_configs,
                ),
                to_pulumi_object_field(
                    "webServerConfigs",
                    &self.r#web_server_configs,
                ),
                to_pulumi_object_field(
                    "webServerNetworkAccessControls",
                    &self.r#web_server_network_access_controls,
                ),
                to_pulumi_object_field(
                    "workloadsConfigs",
                    &self.r#workloads_configs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetEnvironmentConfig {
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
                    r#data_retention_configs: {
                        let field_value = match fields_map.get("dataRetentionConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataRetentionConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_configs: {
                        let field_value = match fields_map.get("databaseConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'databaseConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#encryption_configs: {
                        let field_value = match fields_map.get("encryptionConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryptionConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#maintenance_windows: {
                        let field_value = match fields_map.get("maintenanceWindows") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenanceWindows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#master_authorized_networks_configs: {
                        let field_value = match fields_map.get("masterAuthorizedNetworksConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'masterAuthorizedNetworksConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_configs: {
                        let field_value = match fields_map.get("nodeConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodeConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#private_environment_configs: {
                        let field_value = match fields_map.get("privateEnvironmentConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateEnvironmentConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recovery_configs: {
                        let field_value = match fields_map.get("recoveryConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'recoveryConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#software_configs: {
                        let field_value = match fields_map.get("softwareConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'softwareConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_server_configs: {
                        let field_value = match fields_map.get("webServerConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'webServerConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_server_network_access_controls: {
                        let field_value = match fields_map.get("webServerNetworkAccessControls") {
                            Some(value) => value,
                            None => bail!("Missing field 'webServerNetworkAccessControls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workloads_configs: {
                        let field_value = match fields_map.get("workloadsConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'workloadsConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
