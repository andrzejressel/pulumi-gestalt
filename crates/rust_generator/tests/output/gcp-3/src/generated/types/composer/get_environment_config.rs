#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEnvironmentConfig {
    /// The URI of the Apache Airflow Web UI hosted within the
    /// environment.
    #[builder(into)]
    #[serde(rename = "airflowUri")]
    pub r#airflow_uri: String,
    /// The Cloud Storage prefix of the DAGs for the environment.
    #[builder(into)]
    #[serde(rename = "dagGcsPrefix")]
    pub r#dag_gcs_prefix: String,
    /// The configuration setting for Airflow data retention mechanism. This field is supported for Cloud Composer environments in versions composer-2.0.32-airflow-2.1.4. or newer
    #[builder(into)]
    #[serde(rename = "dataRetentionConfigs")]
    pub r#data_retention_configs: Vec<super::super::types::composer::GetEnvironmentConfigDataRetentionConfig>,
    /// The configuration of Cloud SQL instance that is used by the Apache Airflow software. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    #[serde(rename = "databaseConfigs")]
    pub r#database_configs: Vec<super::super::types::composer::GetEnvironmentConfigDatabaseConfig>,
    /// Optional. If true, builds performed during operations that install Python packages have only private connectivity to Google services. If false, the builds also have access to the internet.
    #[builder(into)]
    #[serde(rename = "enablePrivateBuildsOnly")]
    pub r#enable_private_builds_only: bool,
    /// Optional. If true, a private Composer environment will be created.
    #[builder(into)]
    #[serde(rename = "enablePrivateEnvironment")]
    pub r#enable_private_environment: bool,
    /// The encryption options for the Composer environment and its dependencies.
    #[builder(into)]
    #[serde(rename = "encryptionConfigs")]
    pub r#encryption_configs: Vec<super::super::types::composer::GetEnvironmentConfigEncryptionConfig>,
    /// The size of the Cloud Composer environment. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[builder(into)]
    #[serde(rename = "environmentSize")]
    pub r#environment_size: String,
    /// The Kubernetes Engine cluster used to run the environment.
    #[builder(into)]
    #[serde(rename = "gkeCluster")]
    pub r#gke_cluster: String,
    /// The configuration for Cloud Composer maintenance window.
    #[builder(into)]
    #[serde(rename = "maintenanceWindows")]
    pub r#maintenance_windows: Vec<super::super::types::composer::GetEnvironmentConfigMaintenanceWindow>,
    /// Configuration options for the master authorized networks feature. Enabled master authorized networks will disallow all external traffic to access Kubernetes master through HTTPS except traffic from the given CIDR blocks, Google Compute Engine Public IPs and Google Prod IPs.
    #[builder(into)]
    #[serde(rename = "masterAuthorizedNetworksConfigs")]
    pub r#master_authorized_networks_configs: Vec<super::super::types::composer::GetEnvironmentConfigMasterAuthorizedNetworksConfig>,
    /// The configuration used for the Kubernetes Engine cluster.
    #[builder(into)]
    #[serde(rename = "nodeConfigs")]
    pub r#node_configs: Vec<super::super::types::composer::GetEnvironmentConfigNodeConfig>,
    /// The number of nodes in the Kubernetes Engine cluster that will be used to run this environment. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: i32,
    /// The configuration used for the Private IP Cloud Composer environment.
    #[builder(into)]
    #[serde(rename = "privateEnvironmentConfigs")]
    pub r#private_environment_configs: Vec<super::super::types::composer::GetEnvironmentConfigPrivateEnvironmentConfig>,
    /// The recovery configuration settings for the Cloud Composer environment
    #[builder(into)]
    #[serde(rename = "recoveryConfigs")]
    pub r#recovery_configs: Vec<super::super::types::composer::GetEnvironmentConfigRecoveryConfig>,
    /// Whether high resilience is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.1.15-airflow-*.*.* and newer.
    #[builder(into)]
    #[serde(rename = "resilienceMode")]
    pub r#resilience_mode: String,
    /// The configuration settings for software inside the environment.
    #[builder(into)]
    #[serde(rename = "softwareConfigs")]
    pub r#software_configs: Vec<super::super::types::composer::GetEnvironmentConfigSoftwareConfig>,
    /// The configuration settings for the Airflow web server App Engine instance. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    #[serde(rename = "webServerConfigs")]
    pub r#web_server_configs: Vec<super::super::types::composer::GetEnvironmentConfigWebServerConfig>,
    /// Network-level access control policy for the Airflow web server.
    #[builder(into)]
    #[serde(rename = "webServerNetworkAccessControls")]
    pub r#web_server_network_access_controls: Vec<super::super::types::composer::GetEnvironmentConfigWebServerNetworkAccessControl>,
    /// The workloads configuration settings for the GKE cluster associated with the Cloud Composer environment. Supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[builder(into)]
    #[serde(rename = "workloadsConfigs")]
    pub r#workloads_configs: Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetEnvironmentConfig {
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
                "airflow_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#airflow_uri,
                )
                .await,
            );
            map.insert(
                "dag_gcs_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dag_gcs_prefix,
                )
                .await,
            );
            map.insert(
                "data_retention_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_retention_configs,
                )
                .await,
            );
            map.insert(
                "database_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database_configs,
                )
                .await,
            );
            map.insert(
                "enable_private_builds_only".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_private_builds_only,
                )
                .await,
            );
            map.insert(
                "enable_private_environment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_private_environment,
                )
                .await,
            );
            map.insert(
                "encryption_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption_configs,
                )
                .await,
            );
            map.insert(
                "environment_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#environment_size,
                )
                .await,
            );
            map.insert(
                "gke_cluster".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gke_cluster,
                )
                .await,
            );
            map.insert(
                "maintenance_windows".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maintenance_windows,
                )
                .await,
            );
            map.insert(
                "master_authorized_networks_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#master_authorized_networks_configs,
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
                "private_environment_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_environment_configs,
                )
                .await,
            );
            map.insert(
                "recovery_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#recovery_configs,
                )
                .await,
            );
            map.insert(
                "resilience_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resilience_mode,
                )
                .await,
            );
            map.insert(
                "software_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#software_configs,
                )
                .await,
            );
            map.insert(
                "web_server_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#web_server_configs,
                )
                .await,
            );
            map.insert(
                "web_server_network_access_controls".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#web_server_network_access_controls,
                )
                .await,
            );
            map.insert(
                "workloads_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#workloads_configs,
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
                    r#data_retention_configs: {
                        let field_value = match fields_map.get("data_retention_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_retention_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_configs: {
                        let field_value = match fields_map.get("database_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#encryption_configs: {
                        let field_value = match fields_map.get("encryption_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#maintenance_windows: {
                        let field_value = match fields_map.get("maintenance_windows") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenance_windows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#master_authorized_networks_configs: {
                        let field_value = match fields_map.get("master_authorized_networks_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'master_authorized_networks_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#private_environment_configs: {
                        let field_value = match fields_map.get("private_environment_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_environment_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recovery_configs: {
                        let field_value = match fields_map.get("recovery_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'recovery_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#software_configs: {
                        let field_value = match fields_map.get("software_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'software_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_server_configs: {
                        let field_value = match fields_map.get("web_server_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'web_server_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_server_network_access_controls: {
                        let field_value = match fields_map.get("web_server_network_access_controls") {
                            Some(value) => value,
                            None => bail!("Missing field 'web_server_network_access_controls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workloads_configs: {
                        let field_value = match fields_map.get("workloads_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'workloads_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
