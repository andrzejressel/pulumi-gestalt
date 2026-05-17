#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAutonomousDatabaseProperty {
    /// The amount of storage currently being used for user and system data, in
    /// terabytes.
    #[builder(into)]
    #[serde(rename = "actualUsedDataStorageSizeTb")]
    pub r#actual_used_data_storage_size_tb: f64,
    /// The amount of storage currently allocated for the database tables and
    /// billed for, rounded up in terabytes.
    #[builder(into)]
    #[serde(rename = "allocatedStorageSizeTb")]
    pub r#allocated_storage_size_tb: f64,
    /// Oracle APEX Application Development.
    /// https://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/AutonomousDatabaseApex
    #[builder(into)]
    #[serde(rename = "apexDetails")]
    pub r#apex_details: Vec<super::super::types::oracledatabase::GetAutonomousDatabasePropertyApexDetail>,
    /// This field indicates the status of Data Guard and Access control for the
    /// Autonomous Database. The field's value is null if Data Guard is disabled
    /// or Access Control is disabled. The field's value is TRUE if both Data Guard
    /// and Access Control are enabled, and the Autonomous Database is using
    /// primary IP access control list (ACL) for standby. The field's value is
    /// FALSE if both Data Guard and Access Control are enabled, and the Autonomous
    /// Database is using a different IP access control list (ACL) for standby
    /// compared to primary.
    #[builder(into)]
    #[serde(rename = "arePrimaryAllowlistedIpsUsed")]
    pub r#are_primary_allowlisted_ips_used: bool,
    /// The Autonomous Container Database OCID.
    #[builder(into)]
    #[serde(rename = "autonomousContainerDatabaseId")]
    pub r#autonomous_container_database_id: String,
    /// The list of available Oracle Database upgrade versions for an Autonomous
    /// Database.
    #[builder(into)]
    #[serde(rename = "availableUpgradeVersions")]
    pub r#available_upgrade_versions: Vec<String>,
    /// The retention period for the Autonomous Database. This field is specified
    /// in days, can range from 1 day to 60 days, and has a default value of
    /// 60 days.
    #[builder(into)]
    #[serde(rename = "backupRetentionPeriodDays")]
    pub r#backup_retention_period_days: i32,
    /// The character set for the Autonomous Database. The default is AL32UTF8.
    #[builder(into)]
    #[serde(rename = "characterSet")]
    pub r#character_set: String,
    /// The number of compute servers for the Autonomous Database.
    #[builder(into)]
    #[serde(rename = "computeCount")]
    pub r#compute_count: f64,
    /// The connection string used to connect to the Autonomous Database.
    /// https://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/AutonomousDatabaseConnectionStrings
    #[builder(into)]
    #[serde(rename = "connectionStrings")]
    pub r#connection_strings: Vec<super::super::types::oracledatabase::GetAutonomousDatabasePropertyConnectionString>,
    /// The URLs for accessing Oracle Application Express (APEX) and SQL Developer
    /// Web with a browser from a Compute instance.
    /// https://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/AutonomousDatabaseConnectionUrls
    #[builder(into)]
    #[serde(rename = "connectionUrls")]
    pub r#connection_urls: Vec<super::super::types::oracledatabase::GetAutonomousDatabasePropertyConnectionUrl>,
    /// The list of customer contacts.
    #[builder(into)]
    #[serde(rename = "customerContacts")]
    pub r#customer_contacts: Vec<super::super::types::oracledatabase::GetAutonomousDatabasePropertyCustomerContact>,
    /// The current state of the Data Safe registration for the
    /// Autonomous Database. 
    ///  Possible values:
    ///  DATA_SAFE_STATE_UNSPECIFIED
    /// REGISTERING
    /// REGISTERED
    /// DEREGISTERING
    /// NOT_REGISTERED
    /// FAILED
    #[builder(into)]
    #[serde(rename = "dataSafeState")]
    pub r#data_safe_state: String,
    /// The size of the data stored in the database, in gigabytes.
    #[builder(into)]
    #[serde(rename = "dataStorageSizeGb")]
    pub r#data_storage_size_gb: i32,
    /// The size of the data stored in the database, in terabytes.
    #[builder(into)]
    #[serde(rename = "dataStorageSizeTb")]
    pub r#data_storage_size_tb: i32,
    /// The current state of database management for the Autonomous Database. 
    ///  Possible values:
    ///  DATABASE_MANAGEMENT_STATE_UNSPECIFIED
    /// ENABLING
    /// ENABLED
    /// DISABLING
    /// NOT_ENABLED
    /// FAILED_ENABLING
    /// FAILED_DISABLING
    #[builder(into)]
    #[serde(rename = "databaseManagementState")]
    pub r#database_management_state: String,
    /// The edition of the Autonomous Databases. 
    ///  Possible values:
    ///  DATABASE_EDITION_UNSPECIFIED
    /// STANDARD_EDITION
    /// ENTERPRISE_EDITION
    #[builder(into)]
    #[serde(rename = "dbEdition")]
    pub r#db_edition: String,
    /// The Oracle Database version for the Autonomous Database.
    #[builder(into)]
    #[serde(rename = "dbVersion")]
    pub r#db_version: String,
    /// Possible values:
    ///  DB_WORKLOAD_UNSPECIFIED
    /// OLTP
    /// DW
    /// AJD
    /// APEX
    #[builder(into)]
    #[serde(rename = "dbWorkload")]
    pub r#db_workload: String,
    /// This field indicates the number of seconds of data loss during a Data
    /// Guard failover.
    #[builder(into)]
    #[serde(rename = "failedDataRecoveryDuration")]
    pub r#failed_data_recovery_duration: String,
    /// This field indicates if auto scaling is enabled for the Autonomous Database
    /// CPU core count.
    #[builder(into)]
    #[serde(rename = "isAutoScalingEnabled")]
    pub r#is_auto_scaling_enabled: bool,
    /// This field indicates whether the Autonomous Database has local (in-region)
    /// Data Guard enabled.
    #[builder(into)]
    #[serde(rename = "isLocalDataGuardEnabled")]
    pub r#is_local_data_guard_enabled: bool,
    /// This field indicates if auto scaling is enabled for the Autonomous Database
    /// storage.
    #[builder(into)]
    #[serde(rename = "isStorageAutoScalingEnabled")]
    pub r#is_storage_auto_scaling_enabled: bool,
    /// The license type used for the Autonomous Database. 
    ///  Possible values:
    ///  LICENSE_TYPE_UNSPECIFIED
    /// LICENSE_INCLUDED
    /// BRING_YOUR_OWN_LICENSE
    #[builder(into)]
    #[serde(rename = "licenseType")]
    pub r#license_type: String,
    /// The details of the current lifestyle state of the Autonomous Database.
    #[builder(into)]
    #[serde(rename = "lifecycleDetails")]
    pub r#lifecycle_details: String,
    /// This field indicates the maximum data loss limit for an Autonomous
    /// Database, in seconds.
    #[builder(into)]
    #[serde(rename = "localAdgAutoFailoverMaxDataLossLimit")]
    pub r#local_adg_auto_failover_max_data_loss_limit: i32,
    /// This field indicates the local disaster recovery (DR) type of an
    /// Autonomous Database. 
    ///  Possible values:
    ///  LOCAL_DISASTER_RECOVERY_TYPE_UNSPECIFIED
    /// ADG
    /// BACKUP_BASED
    #[builder(into)]
    #[serde(rename = "localDisasterRecoveryType")]
    pub r#local_disaster_recovery_type: String,
    /// Autonomous Data Guard standby database details.
    /// https://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/AutonomousDatabaseStandbySummary
    #[builder(into)]
    #[serde(rename = "localStandbyDbs")]
    pub r#local_standby_dbs: Vec<super::super::types::oracledatabase::GetAutonomousDatabasePropertyLocalStandbyDb>,
    /// The date and time when maintenance will begin.
    #[builder(into)]
    #[serde(rename = "maintenanceBeginTime")]
    pub r#maintenance_begin_time: String,
    /// The date and time when maintenance will end.
    #[builder(into)]
    #[serde(rename = "maintenanceEndTime")]
    pub r#maintenance_end_time: String,
    /// The maintenance schedule of the Autonomous Database. 
    ///  Possible values:
    ///  MAINTENANCE_SCHEDULE_TYPE_UNSPECIFIED
    /// EARLY
    /// REGULAR
    #[builder(into)]
    #[serde(rename = "maintenanceScheduleType")]
    pub r#maintenance_schedule_type: String,
    /// The amount of memory enabled per ECPU, in gigabytes.
    #[builder(into)]
    #[serde(rename = "memoryPerOracleComputeUnitGbs")]
    pub r#memory_per_oracle_compute_unit_gbs: i32,
    /// The memory assigned to in-memory tables in an Autonomous Database.
    #[builder(into)]
    #[serde(rename = "memoryTableGbs")]
    pub r#memory_table_gbs: i32,
    /// This field specifies if the Autonomous Database requires mTLS connections.
    #[builder(into)]
    #[serde(rename = "mtlsConnectionRequired")]
    pub r#mtls_connection_required: bool,
    /// The national character set for the Autonomous Database. The default is
    /// AL16UTF16.
    #[builder(into)]
    #[serde(rename = "nCharacterSet")]
    pub r#n_character_set: String,
    /// The long term backup schedule of the Autonomous Database.
    #[builder(into)]
    #[serde(rename = "nextLongTermBackupTime")]
    pub r#next_long_term_backup_time: String,
    /// The Oracle Cloud Infrastructure link for the Autonomous Database.
    #[builder(into)]
    #[serde(rename = "ociUrl")]
    pub r#oci_url: String,
    /// OCID of the Autonomous Database.
    /// https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm#Oracle
    #[builder(into)]
    #[serde(rename = "ocid")]
    pub r#ocid: String,
    /// This field indicates the current mode of the Autonomous Database. 
    ///  Possible values:
    ///  OPEN_MODE_UNSPECIFIED
    /// READ_ONLY
    /// READ_WRITE
    #[builder(into)]
    #[serde(rename = "openMode")]
    pub r#open_mode: String,
    /// Possible values:
    ///  OPERATIONS_INSIGHTS_STATE_UNSPECIFIED
    /// ENABLING
    /// ENABLED
    /// DISABLING
    /// NOT_ENABLED
    /// FAILED_ENABLING
    /// FAILED_DISABLING
    #[builder(into)]
    #[serde(rename = "operationsInsightsState")]
    pub r#operations_insights_state: String,
    /// The list of OCIDs of standby databases located in Autonomous Data Guard
    /// remote regions that are associated with the source database.
    #[builder(into)]
    #[serde(rename = "peerDbIds")]
    pub r#peer_db_ids: Vec<String>,
    /// The permission level of the Autonomous Database. 
    ///  Possible values:
    ///  PERMISSION_LEVEL_UNSPECIFIED
    /// RESTRICTED
    /// UNRESTRICTED
    #[builder(into)]
    #[serde(rename = "permissionLevel")]
    pub r#permission_level: String,
    /// The private endpoint for the Autonomous Database.
    #[builder(into)]
    #[serde(rename = "privateEndpoint")]
    pub r#private_endpoint: String,
    /// The private endpoint IP address for the Autonomous Database.
    #[builder(into)]
    #[serde(rename = "privateEndpointIp")]
    pub r#private_endpoint_ip: String,
    /// The private endpoint label for the Autonomous Database.
    #[builder(into)]
    #[serde(rename = "privateEndpointLabel")]
    pub r#private_endpoint_label: String,
    /// The refresh mode of the cloned Autonomous Database. 
    ///  Possible values:
    ///  REFRESHABLE_MODE_UNSPECIFIED
    /// AUTOMATIC
    /// MANUAL
    #[builder(into)]
    #[serde(rename = "refreshableMode")]
    pub r#refreshable_mode: String,
    /// The refresh State of the clone. 
    ///  Possible values:
    ///  REFRESHABLE_STATE_UNSPECIFIED
    /// REFRESHING
    /// NOT_REFRESHING
    #[builder(into)]
    #[serde(rename = "refreshableState")]
    pub r#refreshable_state: String,
    /// The Data Guard role of the Autonomous Database. 
    ///  Possible values:
    ///  ROLE_UNSPECIFIED
    /// PRIMARY
    /// STANDBY
    /// DISABLED_STANDBY
    /// BACKUP_COPY
    /// SNAPSHOT_STANDBY
    #[builder(into)]
    #[serde(rename = "role")]
    pub r#role: String,
    /// The list and details of the scheduled operations of the Autonomous
    /// Database.
    #[builder(into)]
    #[serde(rename = "scheduledOperationDetails")]
    pub r#scheduled_operation_details: Vec<super::super::types::oracledatabase::GetAutonomousDatabasePropertyScheduledOperationDetail>,
    /// The SQL Web Developer URL for the Autonomous Database.
    #[builder(into)]
    #[serde(rename = "sqlWebDeveloperUrl")]
    pub r#sql_web_developer_url: String,
    /// Possible values:
    ///  STATE_UNSPECIFIED
    /// PROVISIONING
    /// AVAILABLE
    /// STOPPING
    /// STOPPED
    /// STARTING
    /// TERMINATING
    /// TERMINATED
    /// UNAVAILABLE
    /// RESTORE_IN_PROGRESS
    /// RESTORE_FAILED
    /// BACKUP_IN_PROGRESS
    /// SCALE_IN_PROGRESS
    /// AVAILABLE_NEEDS_ATTENTION
    /// UPDATING
    /// MAINTENANCE_IN_PROGRESS
    /// RESTARTING
    /// RECREATING
    /// ROLE_CHANGE_IN_PROGRESS
    /// UPGRADING
    /// INACCESSIBLE
    /// STANDBY
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
    /// The list of available regions that can be used to create a clone for the
    /// Autonomous Database.
    #[builder(into)]
    #[serde(rename = "supportedCloneRegions")]
    pub r#supported_clone_regions: Vec<String>,
    /// The storage space used by automatic backups of Autonomous Database, in
    /// gigabytes.
    #[builder(into)]
    #[serde(rename = "totalAutoBackupStorageSizeGbs")]
    pub r#total_auto_backup_storage_size_gbs: f64,
    /// The storage space used by Autonomous Database, in gigabytes.
    #[builder(into)]
    #[serde(rename = "usedDataStorageSizeTbs")]
    pub r#used_data_storage_size_tbs: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAutonomousDatabaseProperty {
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
                "actual_used_data_storage_size_tb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#actual_used_data_storage_size_tb,
                )
                .await,
            );
            map.insert(
                "allocated_storage_size_tb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allocated_storage_size_tb,
                )
                .await,
            );
            map.insert(
                "apex_details".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#apex_details,
                )
                .await,
            );
            map.insert(
                "are_primary_allowlisted_ips_used".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#are_primary_allowlisted_ips_used,
                )
                .await,
            );
            map.insert(
                "autonomous_container_database_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#autonomous_container_database_id,
                )
                .await,
            );
            map.insert(
                "available_upgrade_versions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#available_upgrade_versions,
                )
                .await,
            );
            map.insert(
                "backup_retention_period_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backup_retention_period_days,
                )
                .await,
            );
            map.insert(
                "character_set".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#character_set,
                )
                .await,
            );
            map.insert(
                "compute_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#compute_count,
                )
                .await,
            );
            map.insert(
                "connection_strings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection_strings,
                )
                .await,
            );
            map.insert(
                "connection_urls".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection_urls,
                )
                .await,
            );
            map.insert(
                "customer_contacts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#customer_contacts,
                )
                .await,
            );
            map.insert(
                "data_safe_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_safe_state,
                )
                .await,
            );
            map.insert(
                "data_storage_size_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_storage_size_gb,
                )
                .await,
            );
            map.insert(
                "data_storage_size_tb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_storage_size_tb,
                )
                .await,
            );
            map.insert(
                "database_management_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database_management_state,
                )
                .await,
            );
            map.insert(
                "db_edition".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#db_edition,
                )
                .await,
            );
            map.insert(
                "db_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#db_version,
                )
                .await,
            );
            map.insert(
                "db_workload".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#db_workload,
                )
                .await,
            );
            map.insert(
                "failed_data_recovery_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#failed_data_recovery_duration,
                )
                .await,
            );
            map.insert(
                "is_auto_scaling_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_auto_scaling_enabled,
                )
                .await,
            );
            map.insert(
                "is_local_data_guard_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_local_data_guard_enabled,
                )
                .await,
            );
            map.insert(
                "is_storage_auto_scaling_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_storage_auto_scaling_enabled,
                )
                .await,
            );
            map.insert(
                "license_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#license_type,
                )
                .await,
            );
            map.insert(
                "lifecycle_details".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lifecycle_details,
                )
                .await,
            );
            map.insert(
                "local_adg_auto_failover_max_data_loss_limit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_adg_auto_failover_max_data_loss_limit,
                )
                .await,
            );
            map.insert(
                "local_disaster_recovery_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_disaster_recovery_type,
                )
                .await,
            );
            map.insert(
                "local_standby_dbs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_standby_dbs,
                )
                .await,
            );
            map.insert(
                "maintenance_begin_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maintenance_begin_time,
                )
                .await,
            );
            map.insert(
                "maintenance_end_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maintenance_end_time,
                )
                .await,
            );
            map.insert(
                "maintenance_schedule_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maintenance_schedule_type,
                )
                .await,
            );
            map.insert(
                "memory_per_oracle_compute_unit_gbs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#memory_per_oracle_compute_unit_gbs,
                )
                .await,
            );
            map.insert(
                "memory_table_gbs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#memory_table_gbs,
                )
                .await,
            );
            map.insert(
                "mtls_connection_required".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mtls_connection_required,
                )
                .await,
            );
            map.insert(
                "n_character_set".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#n_character_set,
                )
                .await,
            );
            map.insert(
                "next_long_term_backup_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#next_long_term_backup_time,
                )
                .await,
            );
            map.insert(
                "oci_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#oci_url,
                )
                .await,
            );
            map.insert(
                "ocid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ocid,
                )
                .await,
            );
            map.insert(
                "open_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#open_mode,
                )
                .await,
            );
            map.insert(
                "operations_insights_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#operations_insights_state,
                )
                .await,
            );
            map.insert(
                "peer_db_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#peer_db_ids,
                )
                .await,
            );
            map.insert(
                "permission_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#permission_level,
                )
                .await,
            );
            map.insert(
                "private_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_endpoint,
                )
                .await,
            );
            map.insert(
                "private_endpoint_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_endpoint_ip,
                )
                .await,
            );
            map.insert(
                "private_endpoint_label".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_endpoint_label,
                )
                .await,
            );
            map.insert(
                "refreshable_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#refreshable_mode,
                )
                .await,
            );
            map.insert(
                "refreshable_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#refreshable_state,
                )
                .await,
            );
            map.insert(
                "role".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#role,
                )
                .await,
            );
            map.insert(
                "scheduled_operation_details".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scheduled_operation_details,
                )
                .await,
            );
            map.insert(
                "sql_web_developer_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sql_web_developer_url,
                )
                .await,
            );
            map.insert(
                "state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#state,
                )
                .await,
            );
            map.insert(
                "supported_clone_regions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#supported_clone_regions,
                )
                .await,
            );
            map.insert(
                "total_auto_backup_storage_size_gbs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#total_auto_backup_storage_size_gbs,
                )
                .await,
            );
            map.insert(
                "used_data_storage_size_tbs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#used_data_storage_size_tbs,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAutonomousDatabaseProperty {
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
                    r#actual_used_data_storage_size_tb: {
                        let field_value = match fields_map.get("actual_used_data_storage_size_tb") {
                            Some(value) => value,
                            None => bail!("Missing field 'actual_used_data_storage_size_tb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allocated_storage_size_tb: {
                        let field_value = match fields_map.get("allocated_storage_size_tb") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocated_storage_size_tb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#apex_details: {
                        let field_value = match fields_map.get("apex_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'apex_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#are_primary_allowlisted_ips_used: {
                        let field_value = match fields_map.get("are_primary_allowlisted_ips_used") {
                            Some(value) => value,
                            None => bail!("Missing field 'are_primary_allowlisted_ips_used' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#autonomous_container_database_id: {
                        let field_value = match fields_map.get("autonomous_container_database_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'autonomous_container_database_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_upgrade_versions: {
                        let field_value = match fields_map.get("available_upgrade_versions") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_upgrade_versions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_retention_period_days: {
                        let field_value = match fields_map.get("backup_retention_period_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_retention_period_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#character_set: {
                        let field_value = match fields_map.get("character_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'character_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compute_count: {
                        let field_value = match fields_map.get("compute_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'compute_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_strings: {
                        let field_value = match fields_map.get("connection_strings") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_strings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_urls: {
                        let field_value = match fields_map.get("connection_urls") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_urls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#customer_contacts: {
                        let field_value = match fields_map.get("customer_contacts") {
                            Some(value) => value,
                            None => bail!("Missing field 'customer_contacts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_safe_state: {
                        let field_value = match fields_map.get("data_safe_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_safe_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_storage_size_gb: {
                        let field_value = match fields_map.get("data_storage_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_storage_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_storage_size_tb: {
                        let field_value = match fields_map.get("data_storage_size_tb") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_storage_size_tb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_management_state: {
                        let field_value = match fields_map.get("database_management_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_management_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_edition: {
                        let field_value = match fields_map.get("db_edition") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_edition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_version: {
                        let field_value = match fields_map.get("db_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_workload: {
                        let field_value = match fields_map.get("db_workload") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_workload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failed_data_recovery_duration: {
                        let field_value = match fields_map.get("failed_data_recovery_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'failed_data_recovery_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_auto_scaling_enabled: {
                        let field_value = match fields_map.get("is_auto_scaling_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_auto_scaling_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_local_data_guard_enabled: {
                        let field_value = match fields_map.get("is_local_data_guard_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_local_data_guard_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_storage_auto_scaling_enabled: {
                        let field_value = match fields_map.get("is_storage_auto_scaling_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_storage_auto_scaling_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#license_type: {
                        let field_value = match fields_map.get("license_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'license_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifecycle_details: {
                        let field_value = match fields_map.get("lifecycle_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycle_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_adg_auto_failover_max_data_loss_limit: {
                        let field_value = match fields_map.get("local_adg_auto_failover_max_data_loss_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_adg_auto_failover_max_data_loss_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_disaster_recovery_type: {
                        let field_value = match fields_map.get("local_disaster_recovery_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_disaster_recovery_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_standby_dbs: {
                        let field_value = match fields_map.get("local_standby_dbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_standby_dbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maintenance_begin_time: {
                        let field_value = match fields_map.get("maintenance_begin_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenance_begin_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maintenance_end_time: {
                        let field_value = match fields_map.get("maintenance_end_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenance_end_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maintenance_schedule_type: {
                        let field_value = match fields_map.get("maintenance_schedule_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenance_schedule_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_per_oracle_compute_unit_gbs: {
                        let field_value = match fields_map.get("memory_per_oracle_compute_unit_gbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_per_oracle_compute_unit_gbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_table_gbs: {
                        let field_value = match fields_map.get("memory_table_gbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_table_gbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mtls_connection_required: {
                        let field_value = match fields_map.get("mtls_connection_required") {
                            Some(value) => value,
                            None => bail!("Missing field 'mtls_connection_required' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#n_character_set: {
                        let field_value = match fields_map.get("n_character_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'n_character_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_long_term_backup_time: {
                        let field_value = match fields_map.get("next_long_term_backup_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_long_term_backup_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oci_url: {
                        let field_value = match fields_map.get("oci_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'oci_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ocid: {
                        let field_value = match fields_map.get("ocid") {
                            Some(value) => value,
                            None => bail!("Missing field 'ocid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#open_mode: {
                        let field_value = match fields_map.get("open_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'open_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operations_insights_state: {
                        let field_value = match fields_map.get("operations_insights_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'operations_insights_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peer_db_ids: {
                        let field_value = match fields_map.get("peer_db_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'peer_db_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#permission_level: {
                        let field_value = match fields_map.get("permission_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'permission_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_endpoint: {
                        let field_value = match fields_map.get("private_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_endpoint_ip: {
                        let field_value = match fields_map.get("private_endpoint_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_endpoint_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_endpoint_label: {
                        let field_value = match fields_map.get("private_endpoint_label") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_endpoint_label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#refreshable_mode: {
                        let field_value = match fields_map.get("refreshable_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'refreshable_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#refreshable_state: {
                        let field_value = match fields_map.get("refreshable_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'refreshable_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role: {
                        let field_value = match fields_map.get("role") {
                            Some(value) => value,
                            None => bail!("Missing field 'role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scheduled_operation_details: {
                        let field_value = match fields_map.get("scheduled_operation_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'scheduled_operation_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_web_developer_url: {
                        let field_value = match fields_map.get("sql_web_developer_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'sql_web_developer_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#supported_clone_regions: {
                        let field_value = match fields_map.get("supported_clone_regions") {
                            Some(value) => value,
                            None => bail!("Missing field 'supported_clone_regions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#total_auto_backup_storage_size_gbs: {
                        let field_value = match fields_map.get("total_auto_backup_storage_size_gbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'total_auto_backup_storage_size_gbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#used_data_storage_size_tbs: {
                        let field_value = match fields_map.get("used_data_storage_size_tbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'used_data_storage_size_tbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
