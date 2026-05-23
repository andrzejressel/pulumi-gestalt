#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatabaseInstanceSettings {
    /// This specifies when the instance should be
    /// active. Can be either `ALWAYS`, `NEVER` or `ON_DEMAND`.
    #[builder(into)]
    pub r#activation_policy: Option<String>,
    #[builder(into)]
    pub r#active_directory_config: Option<Box<super::super::types::sql::DatabaseInstanceSettingsActiveDirectoryConfig>>,
    #[builder(into)]
    pub r#advanced_machine_features: Option<Box<super::super::types::sql::DatabaseInstanceSettingsAdvancedMachineFeatures>>,
    /// The availability type of the Cloud SQL
    /// instance, high availability (`REGIONAL`) or single zone (`ZONAL`).' For all instances, ensure that
    /// `settings.backup_configuration.enabled` is set to `true`.
    /// For MySQL instances, ensure that `settings.backup_configuration.binary_log_enabled` is set to `true`.
    /// For Postgres and SQL Server instances, ensure that `settings.backup_configuration.point_in_time_recovery_enabled`
    /// is set to `true`. Defaults to `ZONAL`.
    #[builder(into)]
    pub r#availability_type: Option<String>,
    #[builder(into)]
    pub r#backup_configuration: Option<Box<super::super::types::sql::DatabaseInstanceSettingsBackupConfiguration>>,
    /// The name of server instance collation.
    #[builder(into)]
    pub r#collation: Option<String>,
    /// Control the enforcement of Cloud SQL Auth Proxy or Cloud SQL connectors for all the connections, can be `REQUIRED` or `NOT_REQUIRED`. If enabled, all the direct connections are rejected.
    #[builder(into)]
    pub r#connector_enforcement: Option<String>,
    /// Data cache configurations.
    #[builder(into)]
    pub r#data_cache_config: Option<Box<super::super::types::sql::DatabaseInstanceSettingsDataCacheConfig>>,
    #[builder(into)]
    pub r#database_flags: Option<Vec<super::super::types::sql::DatabaseInstanceSettingsDatabaseFlag>>,
    /// Configuration to protect against accidental instance deletion.
    #[builder(into)]
    pub r#deletion_protection_enabled: Option<bool>,
    #[builder(into)]
    pub r#deny_maintenance_period: Option<Box<super::super::types::sql::DatabaseInstanceSettingsDenyMaintenancePeriod>>,
    /// Enables auto-resizing of the storage size. Defaults to `true`. Note that if `disk_size` is set, future `pulumi up` calls will attempt to delete the instance in order to resize the disk to the value specified in disk_size if it has been resized. To avoid this, ensure that `lifecycle.ignore_changes` is applied to `disk_size`.
    #[builder(into)]
    pub r#disk_autoresize: Option<bool>,
    /// The maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit.
    #[builder(into)]
    pub r#disk_autoresize_limit: Option<i32>,
    /// The size of data disk, in GB. Size of a running instance cannot be reduced but can be increased. The minimum value is 10GB. Note that this value will override the resizing from `disk_autoresize` if that feature is enabled. To avoid this, set `lifecycle.ignore_changes` on this field.
    #[builder(into)]
    pub r#disk_size: Option<i32>,
    /// The type of data disk: PD_SSD or PD_HDD. Defaults to `PD_SSD`.
    #[builder(into)]
    pub r#disk_type: Option<String>,
    /// The edition of the instance, can be `ENTERPRISE` or `ENTERPRISE_PLUS`.
    #[builder(into)]
    pub r#edition: Option<String>,
    /// Enables [Cloud SQL instance integration with Dataplex](https://cloud.google.com/sql/docs/mysql/dataplex-catalog-integration). MySQL, Postgres and SQL Server instances are supported for this feature. Defaults to `false`.
    #[builder(into)]
    pub r#enable_dataplex_integration: Option<bool>,
    /// Enables [Cloud SQL instances to connect to Vertex AI](https://cloud.google.com/sql/docs/postgres/integrate-cloud-sql-with-vertex-ai) and pass requests for real-time predictions and insights. Defaults to `false`.
    #[builder(into)]
    pub r#enable_google_ml_integration: Option<bool>,
    /// Configuration of Query Insights.
    #[builder(into)]
    pub r#insights_config: Option<Box<super::super::types::sql::DatabaseInstanceSettingsInsightsConfig>>,
    #[builder(into)]
    pub r#ip_configuration: Option<Box<super::super::types::sql::DatabaseInstanceSettingsIpConfiguration>>,
    #[builder(into)]
    pub r#location_preference: Option<Box<super::super::types::sql::DatabaseInstanceSettingsLocationPreference>>,
    /// Declares a one-hour maintenance window when an Instance can automatically restart to apply updates. The maintenance window is specified in UTC time.
    #[builder(into)]
    pub r#maintenance_window: Option<Box<super::super::types::sql::DatabaseInstanceSettingsMaintenanceWindow>>,
    #[builder(into)]
    pub r#password_validation_policy: Option<Box<super::super::types::sql::DatabaseInstanceSettingsPasswordValidationPolicy>>,
    /// Pricing plan for this instance, can only be `PER_USE`.
    #[builder(into)]
    pub r#pricing_plan: Option<String>,
    #[builder(into)]
    pub r#sql_server_audit_config: Option<Box<super::super::types::sql::DatabaseInstanceSettingsSqlServerAuditConfig>>,
    /// The machine type to use. See [tiers](https://cloud.google.com/sql/docs/admin-api/v1beta4/tiers)
    /// for more details and supported versions. Postgres supports only shared-core machine types,
    /// and custom machine types such as `db-custom-2-13312`. See the [Custom Machine Type Documentation](https://cloud.google.com/compute/docs/instances/creating-instance-with-custom-machine-type#create) to learn about specifying custom machine types.
    #[builder(into)]
    pub r#tier: String,
    /// The time_zone to be used by the database engine (supported only for SQL Server), in SQL Server timezone format.
    #[builder(into)]
    pub r#time_zone: Option<String>,
    /// A set of key/value user label pairs to assign to the instance.
    #[builder(into)]
    pub r#user_labels: Option<std::collections::HashMap<String, String>>,
    /// Used to make sure changes to the `settings` block are
    /// atomic.
    #[builder(into)]
    pub r#version: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatabaseInstanceSettings {
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
                    "activationPolicy",
                    &self.r#activation_policy,
                ),
                to_pulumi_object_field(
                    "activeDirectoryConfig",
                    &self.r#active_directory_config,
                ),
                to_pulumi_object_field(
                    "advancedMachineFeatures",
                    &self.r#advanced_machine_features,
                ),
                to_pulumi_object_field(
                    "availabilityType",
                    &self.r#availability_type,
                ),
                to_pulumi_object_field(
                    "backupConfiguration",
                    &self.r#backup_configuration,
                ),
                to_pulumi_object_field(
                    "collation",
                    &self.r#collation,
                ),
                to_pulumi_object_field(
                    "connectorEnforcement",
                    &self.r#connector_enforcement,
                ),
                to_pulumi_object_field(
                    "dataCacheConfig",
                    &self.r#data_cache_config,
                ),
                to_pulumi_object_field(
                    "databaseFlags",
                    &self.r#database_flags,
                ),
                to_pulumi_object_field(
                    "deletionProtectionEnabled",
                    &self.r#deletion_protection_enabled,
                ),
                to_pulumi_object_field(
                    "denyMaintenancePeriod",
                    &self.r#deny_maintenance_period,
                ),
                to_pulumi_object_field(
                    "diskAutoresize",
                    &self.r#disk_autoresize,
                ),
                to_pulumi_object_field(
                    "diskAutoresizeLimit",
                    &self.r#disk_autoresize_limit,
                ),
                to_pulumi_object_field(
                    "diskSize",
                    &self.r#disk_size,
                ),
                to_pulumi_object_field(
                    "diskType",
                    &self.r#disk_type,
                ),
                to_pulumi_object_field(
                    "edition",
                    &self.r#edition,
                ),
                to_pulumi_object_field(
                    "enableDataplexIntegration",
                    &self.r#enable_dataplex_integration,
                ),
                to_pulumi_object_field(
                    "enableGoogleMlIntegration",
                    &self.r#enable_google_ml_integration,
                ),
                to_pulumi_object_field(
                    "insightsConfig",
                    &self.r#insights_config,
                ),
                to_pulumi_object_field(
                    "ipConfiguration",
                    &self.r#ip_configuration,
                ),
                to_pulumi_object_field(
                    "locationPreference",
                    &self.r#location_preference,
                ),
                to_pulumi_object_field(
                    "maintenanceWindow",
                    &self.r#maintenance_window,
                ),
                to_pulumi_object_field(
                    "passwordValidationPolicy",
                    &self.r#password_validation_policy,
                ),
                to_pulumi_object_field(
                    "pricingPlan",
                    &self.r#pricing_plan,
                ),
                to_pulumi_object_field(
                    "sqlServerAuditConfig",
                    &self.r#sql_server_audit_config,
                ),
                to_pulumi_object_field(
                    "tier",
                    &self.r#tier,
                ),
                to_pulumi_object_field(
                    "timeZone",
                    &self.r#time_zone,
                ),
                to_pulumi_object_field(
                    "userLabels",
                    &self.r#user_labels,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatabaseInstanceSettings {
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
                    r#activation_policy: {
                        let field_value = match fields_map.get("activationPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'activationPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#active_directory_config: {
                        let field_value = match fields_map.get("activeDirectoryConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'activeDirectoryConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#advanced_machine_features: {
                        let field_value = match fields_map.get("advancedMachineFeatures") {
                            Some(value) => value,
                            None => bail!("Missing field 'advancedMachineFeatures' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#availability_type: {
                        let field_value = match fields_map.get("availabilityType") {
                            Some(value) => value,
                            None => bail!("Missing field 'availabilityType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_configuration: {
                        let field_value = match fields_map.get("backupConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'backupConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#collation: {
                        let field_value = match fields_map.get("collation") {
                            Some(value) => value,
                            None => bail!("Missing field 'collation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connector_enforcement: {
                        let field_value = match fields_map.get("connectorEnforcement") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectorEnforcement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_cache_config: {
                        let field_value = match fields_map.get("dataCacheConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataCacheConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_flags: {
                        let field_value = match fields_map.get("databaseFlags") {
                            Some(value) => value,
                            None => bail!("Missing field 'databaseFlags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deletion_protection_enabled: {
                        let field_value = match fields_map.get("deletionProtectionEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'deletionProtectionEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deny_maintenance_period: {
                        let field_value = match fields_map.get("denyMaintenancePeriod") {
                            Some(value) => value,
                            None => bail!("Missing field 'denyMaintenancePeriod' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_autoresize: {
                        let field_value = match fields_map.get("diskAutoresize") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskAutoresize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_autoresize_limit: {
                        let field_value = match fields_map.get("diskAutoresizeLimit") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskAutoresizeLimit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_size: {
                        let field_value = match fields_map.get("diskSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_type: {
                        let field_value = match fields_map.get("diskType") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#edition: {
                        let field_value = match fields_map.get("edition") {
                            Some(value) => value,
                            None => bail!("Missing field 'edition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_dataplex_integration: {
                        let field_value = match fields_map.get("enableDataplexIntegration") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableDataplexIntegration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_google_ml_integration: {
                        let field_value = match fields_map.get("enableGoogleMlIntegration") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableGoogleMlIntegration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#insights_config: {
                        let field_value = match fields_map.get("insightsConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'insightsConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_configuration: {
                        let field_value = match fields_map.get("ipConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#location_preference: {
                        let field_value = match fields_map.get("locationPreference") {
                            Some(value) => value,
                            None => bail!("Missing field 'locationPreference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#password_validation_policy: {
                        let field_value = match fields_map.get("passwordValidationPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'passwordValidationPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pricing_plan: {
                        let field_value = match fields_map.get("pricingPlan") {
                            Some(value) => value,
                            None => bail!("Missing field 'pricingPlan' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_server_audit_config: {
                        let field_value = match fields_map.get("sqlServerAuditConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqlServerAuditConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier: {
                        let field_value = match fields_map.get("tier") {
                            Some(value) => value,
                            None => bail!("Missing field 'tier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_zone: {
                        let field_value = match fields_map.get("timeZone") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeZone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_labels: {
                        let field_value = match fields_map.get("userLabels") {
                            Some(value) => value,
                            None => bail!("Missing field 'userLabels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
