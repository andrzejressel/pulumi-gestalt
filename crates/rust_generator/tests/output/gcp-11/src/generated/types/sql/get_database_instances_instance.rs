#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDatabaseInstancesInstance {
    /// Available Maintenance versions.
    #[builder(into)]
    #[serde(rename = "availableMaintenanceVersions")]
    pub r#available_maintenance_versions: Vec<String>,
    /// Configuration for creating a new instance as a clone of another instance.
    #[builder(into)]
    #[serde(rename = "clones")]
    pub r#clones: Vec<super::super::types::sql::GetDatabaseInstancesInstanceClone>,
    /// The connection name of the instance to be used in connection strings. For example, when connecting with Cloud SQL Proxy.
    #[builder(into)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: String,
    /// To filter out the Cloud SQL instances which are of the specified database version.
    #[builder(into)]
    #[serde(rename = "databaseVersion")]
    pub r#database_version: String,
    #[builder(into)]
    #[serde(rename = "deletionProtection")]
    pub r#deletion_protection: bool,
    /// The dns name of the instance.
    #[builder(into)]
    #[serde(rename = "dnsName")]
    pub r#dns_name: String,
    #[builder(into)]
    #[serde(rename = "encryptionKeyName")]
    pub r#encryption_key_name: String,
    #[builder(into)]
    #[serde(rename = "firstIpAddress")]
    pub r#first_ip_address: String,
    /// The type of the instance. The valid values are:- 'SQL_INSTANCE_TYPE_UNSPECIFIED', 'CLOUD_SQL_INSTANCE', 'ON_PREMISES_INSTANCE' and 'READ_REPLICA_INSTANCE'.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: String,
    #[builder(into)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Vec<super::super::types::sql::GetDatabaseInstancesInstanceIpAddress>,
    /// Maintenance version.
    #[builder(into)]
    #[serde(rename = "maintenanceVersion")]
    pub r#maintenance_version: String,
    /// The name of the instance that will act as the master in the replication setup. Note, this requires the master to have binary_log_enabled set, as well as existing backups.
    #[builder(into)]
    #[serde(rename = "masterInstanceName")]
    pub r#master_instance_name: String,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: String,
    /// The ID of the project in which the resources belong. If it is not provided, the provider project is used.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: String,
    /// The link to service attachment of PSC instance.
    #[builder(into)]
    #[serde(rename = "pscServiceAttachmentLink")]
    pub r#psc_service_attachment_link: String,
    #[builder(into)]
    #[serde(rename = "publicIpAddress")]
    pub r#public_ip_address: String,
    /// To filter out the Cloud SQL instances which are located in the specified region.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
    /// The configuration for replication.
    #[builder(into)]
    #[serde(rename = "replicaConfigurations")]
    pub r#replica_configurations: Vec<super::super::types::sql::GetDatabaseInstancesInstanceReplicaConfiguration>,
    /// The replicas of the instance.
    #[builder(into)]
    #[serde(rename = "replicaNames")]
    pub r#replica_names: Vec<String>,
    #[builder(into)]
    #[serde(rename = "restoreBackupContexts")]
    pub r#restore_backup_contexts: Vec<super::super::types::sql::GetDatabaseInstancesInstanceRestoreBackupContext>,
    /// Initial root password. Required for MS SQL Server.
    #[builder(into)]
    #[serde(rename = "rootPassword")]
    pub r#root_password: String,
    /// The URI of the created resource.
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: String,
    #[builder(into)]
    #[serde(rename = "serverCaCerts")]
    pub r#server_ca_certs: Vec<super::super::types::sql::GetDatabaseInstancesInstanceServerCaCert>,
    /// The service account email address assigned to the instance.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmailAddress")]
    pub r#service_account_email_address: String,
    /// The settings to use for the database. The configuration is detailed below.
    #[builder(into)]
    #[serde(rename = "settings")]
    pub r#settings: Vec<super::super::types::sql::GetDatabaseInstancesInstanceSetting>,
}
