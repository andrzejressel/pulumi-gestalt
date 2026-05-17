#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCacheRedisConfiguration {
    /// Specifies if Microsoft Entra (AAD) authentication is enabled.
    #[builder(into)]
    #[serde(rename = "activeDirectoryAuthenticationEnabled")]
    pub r#active_directory_authentication_enabled: bool,
    #[builder(into)]
    #[serde(rename = "aofBackupEnabled")]
    pub r#aof_backup_enabled: bool,
    #[builder(into)]
    #[serde(rename = "aofStorageConnectionString0")]
    pub r#aof_storage_connection_string_0: String,
    #[builder(into)]
    #[serde(rename = "aofStorageConnectionString1")]
    pub r#aof_storage_connection_string_1: String,
    #[builder(into)]
    #[serde(rename = "authenticationEnabled")]
    pub r#authentication_enabled: bool,
    #[builder(into)]
    #[serde(rename = "dataPersistenceAuthenticationMethod")]
    pub r#data_persistence_authentication_method: String,
    #[builder(into)]
    #[serde(rename = "maxclients")]
    pub r#maxclients: i32,
    /// Value in megabytes reserved to accommodate for memory fragmentation.
    #[builder(into)]
    #[serde(rename = "maxfragmentationmemoryReserved")]
    pub r#maxfragmentationmemory_reserved: i32,
    /// The max-memory delta for this Redis instance.
    #[builder(into)]
    #[serde(rename = "maxmemoryDelta")]
    pub r#maxmemory_delta: i32,
    /// How Redis will select what to remove when `maxmemory` is reached.
    #[builder(into)]
    #[serde(rename = "maxmemoryPolicy")]
    pub r#maxmemory_policy: String,
    /// The value in megabytes reserved for non-cache usage e.g. failover
    #[builder(into)]
    #[serde(rename = "maxmemoryReserved")]
    pub r#maxmemory_reserved: i32,
    #[builder(into)]
    #[serde(rename = "notifyKeyspaceEvents")]
    pub r#notify_keyspace_events: String,
    /// Is Backup Enabled? Only supported on Premium SKUs.
    #[builder(into)]
    #[serde(rename = "rdbBackupEnabled")]
    pub r#rdb_backup_enabled: bool,
    /// The Backup Frequency in Minutes. Only supported on Premium SKUs.
    #[builder(into)]
    #[serde(rename = "rdbBackupFrequency")]
    pub r#rdb_backup_frequency: i32,
    /// The maximum number of snapshots that can be created as a backup.
    #[builder(into)]
    #[serde(rename = "rdbBackupMaxSnapshotCount")]
    pub r#rdb_backup_max_snapshot_count: i32,
    /// The Connection String to the Storage Account. Only supported for Premium SKUs.
    #[builder(into)]
    #[serde(rename = "rdbStorageConnectionString")]
    pub r#rdb_storage_connection_string: String,
    /// The ID of the Subscription containing the Storage Account.
    #[builder(into)]
    #[serde(rename = "storageAccountSubscriptionId")]
    pub r#storage_account_subscription_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCacheRedisConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "active_directory_authentication_enabled",
                    &self.r#active_directory_authentication_enabled,
                ),
                to_pulumi_object_field(
                    "aof_backup_enabled",
                    &self.r#aof_backup_enabled,
                ),
                to_pulumi_object_field(
                    "aof_storage_connection_string_0",
                    &self.r#aof_storage_connection_string_0,
                ),
                to_pulumi_object_field(
                    "aof_storage_connection_string_1",
                    &self.r#aof_storage_connection_string_1,
                ),
                to_pulumi_object_field(
                    "authentication_enabled",
                    &self.r#authentication_enabled,
                ),
                to_pulumi_object_field(
                    "data_persistence_authentication_method",
                    &self.r#data_persistence_authentication_method,
                ),
                to_pulumi_object_field(
                    "maxclients",
                    &self.r#maxclients,
                ),
                to_pulumi_object_field(
                    "maxfragmentationmemory_reserved",
                    &self.r#maxfragmentationmemory_reserved,
                ),
                to_pulumi_object_field(
                    "maxmemory_delta",
                    &self.r#maxmemory_delta,
                ),
                to_pulumi_object_field(
                    "maxmemory_policy",
                    &self.r#maxmemory_policy,
                ),
                to_pulumi_object_field(
                    "maxmemory_reserved",
                    &self.r#maxmemory_reserved,
                ),
                to_pulumi_object_field(
                    "notify_keyspace_events",
                    &self.r#notify_keyspace_events,
                ),
                to_pulumi_object_field(
                    "rdb_backup_enabled",
                    &self.r#rdb_backup_enabled,
                ),
                to_pulumi_object_field(
                    "rdb_backup_frequency",
                    &self.r#rdb_backup_frequency,
                ),
                to_pulumi_object_field(
                    "rdb_backup_max_snapshot_count",
                    &self.r#rdb_backup_max_snapshot_count,
                ),
                to_pulumi_object_field(
                    "rdb_storage_connection_string",
                    &self.r#rdb_storage_connection_string,
                ),
                to_pulumi_object_field(
                    "storage_account_subscription_id",
                    &self.r#storage_account_subscription_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCacheRedisConfiguration {
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
                    r#active_directory_authentication_enabled: {
                        let field_value = match fields_map.get("active_directory_authentication_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'active_directory_authentication_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aof_backup_enabled: {
                        let field_value = match fields_map.get("aof_backup_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'aof_backup_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aof_storage_connection_string_0: {
                        let field_value = match fields_map.get("aof_storage_connection_string_0") {
                            Some(value) => value,
                            None => bail!("Missing field 'aof_storage_connection_string_0' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aof_storage_connection_string_1: {
                        let field_value = match fields_map.get("aof_storage_connection_string_1") {
                            Some(value) => value,
                            None => bail!("Missing field 'aof_storage_connection_string_1' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authentication_enabled: {
                        let field_value = match fields_map.get("authentication_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_persistence_authentication_method: {
                        let field_value = match fields_map.get("data_persistence_authentication_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_persistence_authentication_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maxclients: {
                        let field_value = match fields_map.get("maxclients") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxclients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maxfragmentationmemory_reserved: {
                        let field_value = match fields_map.get("maxfragmentationmemory_reserved") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxfragmentationmemory_reserved' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maxmemory_delta: {
                        let field_value = match fields_map.get("maxmemory_delta") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxmemory_delta' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maxmemory_policy: {
                        let field_value = match fields_map.get("maxmemory_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxmemory_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maxmemory_reserved: {
                        let field_value = match fields_map.get("maxmemory_reserved") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxmemory_reserved' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notify_keyspace_events: {
                        let field_value = match fields_map.get("notify_keyspace_events") {
                            Some(value) => value,
                            None => bail!("Missing field 'notify_keyspace_events' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rdb_backup_enabled: {
                        let field_value = match fields_map.get("rdb_backup_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'rdb_backup_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rdb_backup_frequency: {
                        let field_value = match fields_map.get("rdb_backup_frequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'rdb_backup_frequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rdb_backup_max_snapshot_count: {
                        let field_value = match fields_map.get("rdb_backup_max_snapshot_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'rdb_backup_max_snapshot_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rdb_storage_connection_string: {
                        let field_value = match fields_map.get("rdb_storage_connection_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'rdb_storage_connection_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_subscription_id: {
                        let field_value = match fields_map.get("storage_account_subscription_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_account_subscription_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
