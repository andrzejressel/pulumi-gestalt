#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatabaseInstanceSettings {
    /// This specifies when the instance should be
    /// active. Can be either `ALWAYS`, `NEVER` or `ON_DEMAND`.
    #[builder(into)]
    #[serde(rename = "activationPolicy")]
    pub r#activation_policy: Option<String>,
    #[builder(into)]
    #[serde(rename = "activeDirectoryConfig")]
    pub r#active_directory_config: Option<Box<super::super::types::sql::DatabaseInstanceSettingsActiveDirectoryConfig>>,
    #[builder(into)]
    #[serde(rename = "advancedMachineFeatures")]
    pub r#advanced_machine_features: Option<Box<super::super::types::sql::DatabaseInstanceSettingsAdvancedMachineFeatures>>,
    /// The availability type of the Cloud SQL
    /// instance, high availability (`REGIONAL`) or single zone (`ZONAL`).' For all instances, ensure that
    /// `settings.backup_configuration.enabled` is set to `true`.
    /// For MySQL instances, ensure that `settings.backup_configuration.binary_log_enabled` is set to `true`.
    /// For Postgres and SQL Server instances, ensure that `settings.backup_configuration.point_in_time_recovery_enabled`
    /// is set to `true`. Defaults to `ZONAL`.
    #[builder(into)]
    #[serde(rename = "availabilityType")]
    pub r#availability_type: Option<String>,
    #[builder(into)]
    #[serde(rename = "backupConfiguration")]
    pub r#backup_configuration: Option<Box<super::super::types::sql::DatabaseInstanceSettingsBackupConfiguration>>,
    /// The name of server instance collation.
    #[builder(into)]
    #[serde(rename = "collation")]
    pub r#collation: Option<String>,
    /// Control the enforcement of Cloud SQL Auth Proxy or Cloud SQL connectors for all the connections, can be `REQUIRED` or `NOT_REQUIRED`. If enabled, all the direct connections are rejected.
    #[builder(into)]
    #[serde(rename = "connectorEnforcement")]
    pub r#connector_enforcement: Option<String>,
    /// Data cache configurations.
    #[builder(into)]
    #[serde(rename = "dataCacheConfig")]
    pub r#data_cache_config: Option<Box<super::super::types::sql::DatabaseInstanceSettingsDataCacheConfig>>,
    #[builder(into)]
    #[serde(rename = "databaseFlags")]
    pub r#database_flags: Option<Vec<super::super::types::sql::DatabaseInstanceSettingsDatabaseFlag>>,
    /// Configuration to protect against accidental instance deletion.
    #[builder(into)]
    #[serde(rename = "deletionProtectionEnabled")]
    pub r#deletion_protection_enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "denyMaintenancePeriod")]
    pub r#deny_maintenance_period: Option<Box<super::super::types::sql::DatabaseInstanceSettingsDenyMaintenancePeriod>>,
    /// Enables auto-resizing of the storage size. Defaults to `true`. Note that if `disk_size` is set, future `pulumi up` calls will attempt to delete the instance in order to resize the disk to the value specified in disk_size if it has been resized. To avoid this, ensure that `lifecycle.ignore_changes` is applied to `disk_size`.
    #[builder(into)]
    #[serde(rename = "diskAutoresize")]
    pub r#disk_autoresize: Option<bool>,
    /// The maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit.
    #[builder(into)]
    #[serde(rename = "diskAutoresizeLimit")]
    pub r#disk_autoresize_limit: Option<i32>,
    /// The size of data disk, in GB. Size of a running instance cannot be reduced but can be increased. The minimum value is 10GB. Note that this value will override the resizing from `disk_autoresize` if that feature is enabled. To avoid this, set `lifecycle.ignore_changes` on this field.
    #[builder(into)]
    #[serde(rename = "diskSize")]
    pub r#disk_size: Option<i32>,
    /// The type of data disk: PD_SSD or PD_HDD. Defaults to `PD_SSD`.
    #[builder(into)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Option<String>,
    /// The edition of the instance, can be `ENTERPRISE` or `ENTERPRISE_PLUS`.
    #[builder(into)]
    #[serde(rename = "edition")]
    pub r#edition: Option<String>,
    /// Enables [Cloud SQL instance integration with Dataplex](https://cloud.google.com/sql/docs/mysql/dataplex-catalog-integration). MySQL, Postgres and SQL Server instances are supported for this feature. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enableDataplexIntegration")]
    pub r#enable_dataplex_integration: Option<bool>,
    /// Enables [Cloud SQL instances to connect to Vertex AI](https://cloud.google.com/sql/docs/postgres/integrate-cloud-sql-with-vertex-ai) and pass requests for real-time predictions and insights. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enableGoogleMlIntegration")]
    pub r#enable_google_ml_integration: Option<bool>,
    /// Configuration of Query Insights.
    #[builder(into)]
    #[serde(rename = "insightsConfig")]
    pub r#insights_config: Option<Box<super::super::types::sql::DatabaseInstanceSettingsInsightsConfig>>,
    #[builder(into)]
    #[serde(rename = "ipConfiguration")]
    pub r#ip_configuration: Option<Box<super::super::types::sql::DatabaseInstanceSettingsIpConfiguration>>,
    #[builder(into)]
    #[serde(rename = "locationPreference")]
    pub r#location_preference: Option<Box<super::super::types::sql::DatabaseInstanceSettingsLocationPreference>>,
    /// Declares a one-hour maintenance window when an Instance can automatically restart to apply updates. The maintenance window is specified in UTC time.
    #[builder(into)]
    #[serde(rename = "maintenanceWindow")]
    pub r#maintenance_window: Option<Box<super::super::types::sql::DatabaseInstanceSettingsMaintenanceWindow>>,
    #[builder(into)]
    #[serde(rename = "passwordValidationPolicy")]
    pub r#password_validation_policy: Option<Box<super::super::types::sql::DatabaseInstanceSettingsPasswordValidationPolicy>>,
    /// Pricing plan for this instance, can only be `PER_USE`.
    #[builder(into)]
    #[serde(rename = "pricingPlan")]
    pub r#pricing_plan: Option<String>,
    #[builder(into)]
    #[serde(rename = "sqlServerAuditConfig")]
    pub r#sql_server_audit_config: Option<Box<super::super::types::sql::DatabaseInstanceSettingsSqlServerAuditConfig>>,
    /// The machine type to use. See [tiers](https://cloud.google.com/sql/docs/admin-api/v1beta4/tiers)
    /// for more details and supported versions. Postgres supports only shared-core machine types,
    /// and custom machine types such as `db-custom-2-13312`. See the [Custom Machine Type Documentation](https://cloud.google.com/compute/docs/instances/creating-instance-with-custom-machine-type#create) to learn about specifying custom machine types.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: String,
    /// The time_zone to be used by the database engine (supported only for SQL Server), in SQL Server timezone format.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Option<String>,
    /// A set of key/value user label pairs to assign to the instance.
    #[builder(into)]
    #[serde(rename = "userLabels")]
    pub r#user_labels: Option<std::collections::HashMap<String, String>>,
    /// Used to make sure changes to the `settings` block are
    /// atomic.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<i32>,
}
