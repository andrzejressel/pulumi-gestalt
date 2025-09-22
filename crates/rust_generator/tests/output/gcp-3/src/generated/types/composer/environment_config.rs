#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
