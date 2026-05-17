#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDatabaseInstanceSetting {
    /// This specifies when the instance should be active. Can be either ALWAYS, NEVER or ON_DEMAND.
    #[builder(into)]
    #[serde(rename = "activationPolicy")]
    pub r#activation_policy: String,
    #[builder(into)]
    #[serde(rename = "activeDirectoryConfigs")]
    pub r#active_directory_configs: Vec<super::super::types::sql::GetDatabaseInstanceSettingActiveDirectoryConfig>,
    #[builder(into)]
    #[serde(rename = "advancedMachineFeatures")]
    pub r#advanced_machine_features: Vec<super::super::types::sql::GetDatabaseInstanceSettingAdvancedMachineFeature>,
    /// The availability type of the Cloud SQL instance, high availability
    /// (REGIONAL) or single zone (ZONAL). For all instances, ensure that
    /// settings.backup_configuration.enabled is set to true.
    /// For MySQL instances, ensure that settings.backup_configuration.binary_log_enabled is set to true.
    /// For Postgres instances, ensure that settings.backup_configuration.point_in_time_recovery_enabled
    /// is set to true. Defaults to ZONAL.
    #[builder(into)]
    #[serde(rename = "availabilityType")]
    pub r#availability_type: String,
    #[builder(into)]
    #[serde(rename = "backupConfigurations")]
    pub r#backup_configurations: Vec<super::super::types::sql::GetDatabaseInstanceSettingBackupConfiguration>,
    /// The name of server instance collation.
    #[builder(into)]
    #[serde(rename = "collation")]
    pub r#collation: String,
    /// Enables the enforcement of Cloud SQL Auth Proxy or Cloud SQL connectors for all the connections. If enabled, all the direct connections are rejected.
    #[builder(into)]
    #[serde(rename = "connectorEnforcement")]
    pub r#connector_enforcement: String,
    /// Data cache configurations.
    #[builder(into)]
    #[serde(rename = "dataCacheConfigs")]
    pub r#data_cache_configs: Vec<super::super::types::sql::GetDatabaseInstanceSettingDataCacheConfig>,
    #[builder(into)]
    #[serde(rename = "databaseFlags")]
    pub r#database_flags: Vec<super::super::types::sql::GetDatabaseInstanceSettingDatabaseFlag>,
    /// Configuration to protect against accidental instance deletion.
    #[builder(into)]
    #[serde(rename = "deletionProtectionEnabled")]
    pub r#deletion_protection_enabled: bool,
    #[builder(into)]
    #[serde(rename = "denyMaintenancePeriods")]
    pub r#deny_maintenance_periods: Vec<super::super::types::sql::GetDatabaseInstanceSettingDenyMaintenancePeriod>,
    /// Enables auto-resizing of the storage size. Defaults to true.
    #[builder(into)]
    #[serde(rename = "diskAutoresize")]
    pub r#disk_autoresize: bool,
    /// The maximum size, in GB, to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit.
    #[builder(into)]
    #[serde(rename = "diskAutoresizeLimit")]
    pub r#disk_autoresize_limit: i32,
    /// The size of data disk, in GB. Size of a running instance cannot be reduced but can be increased. The minimum value is 10GB.
    #[builder(into)]
    #[serde(rename = "diskSize")]
    pub r#disk_size: i32,
    /// The type of data disk: PD_SSD or PD_HDD. Defaults to PD_SSD.
    #[builder(into)]
    #[serde(rename = "diskType")]
    pub r#disk_type: String,
    /// The edition of the instance, can be ENTERPRISE or ENTERPRISE_PLUS.
    #[builder(into)]
    #[serde(rename = "edition")]
    pub r#edition: String,
    /// Enables Dataplex Integration.
    #[builder(into)]
    #[serde(rename = "enableDataplexIntegration")]
    pub r#enable_dataplex_integration: bool,
    /// Enables Vertex AI Integration.
    #[builder(into)]
    #[serde(rename = "enableGoogleMlIntegration")]
    pub r#enable_google_ml_integration: bool,
    /// Configuration of Query Insights.
    #[builder(into)]
    #[serde(rename = "insightsConfigs")]
    pub r#insights_configs: Vec<super::super::types::sql::GetDatabaseInstanceSettingInsightsConfig>,
    #[builder(into)]
    #[serde(rename = "ipConfigurations")]
    pub r#ip_configurations: Vec<super::super::types::sql::GetDatabaseInstanceSettingIpConfiguration>,
    #[builder(into)]
    #[serde(rename = "locationPreferences")]
    pub r#location_preferences: Vec<super::super::types::sql::GetDatabaseInstanceSettingLocationPreference>,
    /// Declares a one-hour maintenance window when an Instance can automatically restart to apply updates. The maintenance window is specified in UTC time.
    #[builder(into)]
    #[serde(rename = "maintenanceWindows")]
    pub r#maintenance_windows: Vec<super::super::types::sql::GetDatabaseInstanceSettingMaintenanceWindow>,
    #[builder(into)]
    #[serde(rename = "passwordValidationPolicies")]
    pub r#password_validation_policies: Vec<super::super::types::sql::GetDatabaseInstanceSettingPasswordValidationPolicy>,
    /// Pricing plan for this instance, can only be PER_USE.
    #[builder(into)]
    #[serde(rename = "pricingPlan")]
    pub r#pricing_plan: String,
    #[builder(into)]
    #[serde(rename = "sqlServerAuditConfigs")]
    pub r#sql_server_audit_configs: Vec<super::super::types::sql::GetDatabaseInstanceSettingSqlServerAuditConfig>,
    /// The machine type to use. See tiers for more details and supported versions. Postgres supports only shared-core machine types, and custom machine types such as db-custom-2-13312. See the Custom Machine Type Documentation to learn about specifying custom machine types.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: String,
    /// The time_zone to be used by the database engine (supported only for SQL Server), in SQL Server timezone format.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: String,
    /// A set of key/value user label pairs to assign to the instance.
    #[builder(into)]
    #[serde(rename = "userLabels")]
    pub r#user_labels: std::collections::HashMap<String, String>,
    /// Used to make sure changes to the settings block are atomic.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDatabaseInstanceSetting {
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
                    "activation_policy",
                    &self.r#activation_policy,
                ),
                to_pulumi_object_field(
                    "active_directory_configs",
                    &self.r#active_directory_configs,
                ),
                to_pulumi_object_field(
                    "advanced_machine_features",
                    &self.r#advanced_machine_features,
                ),
                to_pulumi_object_field(
                    "availability_type",
                    &self.r#availability_type,
                ),
                to_pulumi_object_field(
                    "backup_configurations",
                    &self.r#backup_configurations,
                ),
                to_pulumi_object_field(
                    "collation",
                    &self.r#collation,
                ),
                to_pulumi_object_field(
                    "connector_enforcement",
                    &self.r#connector_enforcement,
                ),
                to_pulumi_object_field(
                    "data_cache_configs",
                    &self.r#data_cache_configs,
                ),
                to_pulumi_object_field(
                    "database_flags",
                    &self.r#database_flags,
                ),
                to_pulumi_object_field(
                    "deletion_protection_enabled",
                    &self.r#deletion_protection_enabled,
                ),
                to_pulumi_object_field(
                    "deny_maintenance_periods",
                    &self.r#deny_maintenance_periods,
                ),
                to_pulumi_object_field(
                    "disk_autoresize",
                    &self.r#disk_autoresize,
                ),
                to_pulumi_object_field(
                    "disk_autoresize_limit",
                    &self.r#disk_autoresize_limit,
                ),
                to_pulumi_object_field(
                    "disk_size",
                    &self.r#disk_size,
                ),
                to_pulumi_object_field(
                    "disk_type",
                    &self.r#disk_type,
                ),
                to_pulumi_object_field(
                    "edition",
                    &self.r#edition,
                ),
                to_pulumi_object_field(
                    "enable_dataplex_integration",
                    &self.r#enable_dataplex_integration,
                ),
                to_pulumi_object_field(
                    "enable_google_ml_integration",
                    &self.r#enable_google_ml_integration,
                ),
                to_pulumi_object_field(
                    "insights_configs",
                    &self.r#insights_configs,
                ),
                to_pulumi_object_field(
                    "ip_configurations",
                    &self.r#ip_configurations,
                ),
                to_pulumi_object_field(
                    "location_preferences",
                    &self.r#location_preferences,
                ),
                to_pulumi_object_field(
                    "maintenance_windows",
                    &self.r#maintenance_windows,
                ),
                to_pulumi_object_field(
                    "password_validation_policies",
                    &self.r#password_validation_policies,
                ),
                to_pulumi_object_field(
                    "pricing_plan",
                    &self.r#pricing_plan,
                ),
                to_pulumi_object_field(
                    "sql_server_audit_configs",
                    &self.r#sql_server_audit_configs,
                ),
                to_pulumi_object_field(
                    "tier",
                    &self.r#tier,
                ),
                to_pulumi_object_field(
                    "time_zone",
                    &self.r#time_zone,
                ),
                to_pulumi_object_field(
                    "user_labels",
                    &self.r#user_labels,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDatabaseInstanceSetting {
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
                        let field_value = match fields_map.get("activation_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'activation_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#active_directory_configs: {
                        let field_value = match fields_map.get("active_directory_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'active_directory_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#advanced_machine_features: {
                        let field_value = match fields_map.get("advanced_machine_features") {
                            Some(value) => value,
                            None => bail!("Missing field 'advanced_machine_features' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#availability_type: {
                        let field_value = match fields_map.get("availability_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'availability_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_configurations: {
                        let field_value = match fields_map.get("backup_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("connector_enforcement") {
                            Some(value) => value,
                            None => bail!("Missing field 'connector_enforcement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_cache_configs: {
                        let field_value = match fields_map.get("data_cache_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_cache_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_flags: {
                        let field_value = match fields_map.get("database_flags") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_flags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deletion_protection_enabled: {
                        let field_value = match fields_map.get("deletion_protection_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'deletion_protection_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deny_maintenance_periods: {
                        let field_value = match fields_map.get("deny_maintenance_periods") {
                            Some(value) => value,
                            None => bail!("Missing field 'deny_maintenance_periods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_autoresize: {
                        let field_value = match fields_map.get("disk_autoresize") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_autoresize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_autoresize_limit: {
                        let field_value = match fields_map.get("disk_autoresize_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_autoresize_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_size: {
                        let field_value = match fields_map.get("disk_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_type: {
                        let field_value = match fields_map.get("disk_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("enable_dataplex_integration") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_dataplex_integration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_google_ml_integration: {
                        let field_value = match fields_map.get("enable_google_ml_integration") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_google_ml_integration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#insights_configs: {
                        let field_value = match fields_map.get("insights_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'insights_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_configurations: {
                        let field_value = match fields_map.get("ip_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#location_preferences: {
                        let field_value = match fields_map.get("location_preferences") {
                            Some(value) => value,
                            None => bail!("Missing field 'location_preferences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#password_validation_policies: {
                        let field_value = match fields_map.get("password_validation_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'password_validation_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pricing_plan: {
                        let field_value = match fields_map.get("pricing_plan") {
                            Some(value) => value,
                            None => bail!("Missing field 'pricing_plan' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_server_audit_configs: {
                        let field_value = match fields_map.get("sql_server_audit_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'sql_server_audit_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("time_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_labels: {
                        let field_value = match fields_map.get("user_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
