#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CacheRedisConfiguration {
    /// Enable Microsoft Entra (AAD) authentication. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "activeDirectoryAuthenticationEnabled")]
    pub r#active_directory_authentication_enabled: Option<bool>,
    /// Enable or disable AOF persistence for this Redis Cache. Defaults to `false`.
    /// 
    /// > **NOTE:** `aof_backup_enabled` can only be set when SKU is `Premium`.
    #[builder(into)]
    #[serde(rename = "aofBackupEnabled")]
    pub r#aof_backup_enabled: Option<bool>,
    /// First Storage Account connection string for AOF persistence.
    #[builder(into)]
    #[serde(rename = "aofStorageConnectionString0")]
    pub r#aof_storage_connection_string_0: Option<String>,
    /// Second Storage Account connection string for AOF persistence.
    /// 
    /// Example usage:
    /// 
    #[builder(into)]
    #[serde(rename = "aofStorageConnectionString1")]
    pub r#aof_storage_connection_string_1: Option<String>,
    /// If set to `false`, the Redis instance will be accessible without authentication. Defaults to `true`.
    /// 
    /// > **NOTE:** `authentication_enabled` can only be set to `false` if a `subnet_id` is specified; and only works if there aren't existing instances within the subnet with `authentication_enabled` set to `true`.
    #[builder(into)]
    #[serde(rename = "authenticationEnabled")]
    pub r#authentication_enabled: Option<bool>,
    /// Preferred auth method to communicate to storage account used for data persistence. Possible values are `SAS` and `ManagedIdentity`.
    #[builder(into)]
    #[serde(rename = "dataPersistenceAuthenticationMethod")]
    pub r#data_persistence_authentication_method: Option<String>,
    /// Returns the max number of connected clients at the same time.
    #[builder(into)]
    #[serde(rename = "maxclients")]
    pub r#maxclients: Option<i32>,
    /// Value in megabytes reserved to accommodate for memory fragmentation. Defaults are shown below.
    #[builder(into)]
    #[serde(rename = "maxfragmentationmemoryReserved")]
    pub r#maxfragmentationmemory_reserved: Option<i32>,
    /// The max-memory delta for this Redis instance. Defaults are shown below.
    #[builder(into)]
    #[serde(rename = "maxmemoryDelta")]
    pub r#maxmemory_delta: Option<i32>,
    /// How Redis will select what to remove when `maxmemory` is reached. Defaults to `volatile-lru`.
    #[builder(into)]
    #[serde(rename = "maxmemoryPolicy")]
    pub r#maxmemory_policy: Option<String>,
    /// Value in megabytes reserved for non-cache usage e.g. failover. Defaults are shown below.
    #[builder(into)]
    #[serde(rename = "maxmemoryReserved")]
    pub r#maxmemory_reserved: Option<i32>,
    /// Keyspace notifications allows clients to subscribe to Pub/Sub channels in order to receive events affecting the Redis data set in some way. [Reference](https://redis.io/topics/notifications#configuration)
    /// 
    #[builder(into)]
    #[serde(rename = "notifyKeyspaceEvents")]
    pub r#notify_keyspace_events: Option<String>,
    /// Is Backup Enabled? Only supported on Premium SKUs. Defaults to `false`.
    /// 
    /// > **NOTE:** If `rdb_backup_enabled` set to `true`, `rdb_storage_connection_string` must also be set.
    #[builder(into)]
    #[serde(rename = "rdbBackupEnabled")]
    pub r#rdb_backup_enabled: Option<bool>,
    /// The Backup Frequency in Minutes. Only supported on Premium SKUs. Possible values are: `15`, `30`, `60`, `360`, `720` and `1440`.
    #[builder(into)]
    #[serde(rename = "rdbBackupFrequency")]
    pub r#rdb_backup_frequency: Option<i32>,
    /// The maximum number of snapshots to create as a backup. Only supported for Premium SKUs.
    #[builder(into)]
    #[serde(rename = "rdbBackupMaxSnapshotCount")]
    pub r#rdb_backup_max_snapshot_count: Option<i32>,
    /// The Connection String to the Storage Account. Only supported for Premium SKUs. In the format: `DefaultEndpointsProtocol=https;BlobEndpoint=${azurerm_storage_account.example.primary_blob_endpoint};AccountName=${azurerm_storage_account.example.name};AccountKey=${azurerm_storage_account.example.primary_access_key}`.
    /// 
    /// > **NOTE:** There's a bug in the Redis API where the original storage connection string isn't being returned, which [is being tracked in this issue](https://github.com/Azure/azure-rest-api-specs/issues/3037). In the interim you can use [the `ignoreChanges` attribute to ignore changes to this field](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) e.g.:
    #[builder(into)]
    #[serde(rename = "rdbStorageConnectionString")]
    pub r#rdb_storage_connection_string: Option<String>,
    /// The ID of the Subscription containing the Storage Account.
    /// 
    /// ```yaml
    /// resources:
    ///   example:
    ///     type: azure:redis:Cache
    ///     properties:
    ///       ignoreChanges:
    ///         - ${redisConfiguration[0].rdbStorageConnectionString}
    /// ```
    #[builder(into)]
    #[serde(rename = "storageAccountSubscriptionId")]
    pub r#storage_account_subscription_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CacheRedisConfiguration {
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
                    "activeDirectoryAuthenticationEnabled",
                    &self.r#active_directory_authentication_enabled,
                ),
                to_pulumi_object_field(
                    "aofBackupEnabled",
                    &self.r#aof_backup_enabled,
                ),
                to_pulumi_object_field(
                    "aofStorageConnectionString0",
                    &self.r#aof_storage_connection_string_0,
                ),
                to_pulumi_object_field(
                    "aofStorageConnectionString1",
                    &self.r#aof_storage_connection_string_1,
                ),
                to_pulumi_object_field(
                    "authenticationEnabled",
                    &self.r#authentication_enabled,
                ),
                to_pulumi_object_field(
                    "dataPersistenceAuthenticationMethod",
                    &self.r#data_persistence_authentication_method,
                ),
                to_pulumi_object_field(
                    "maxclients",
                    &self.r#maxclients,
                ),
                to_pulumi_object_field(
                    "maxfragmentationmemoryReserved",
                    &self.r#maxfragmentationmemory_reserved,
                ),
                to_pulumi_object_field(
                    "maxmemoryDelta",
                    &self.r#maxmemory_delta,
                ),
                to_pulumi_object_field(
                    "maxmemoryPolicy",
                    &self.r#maxmemory_policy,
                ),
                to_pulumi_object_field(
                    "maxmemoryReserved",
                    &self.r#maxmemory_reserved,
                ),
                to_pulumi_object_field(
                    "notifyKeyspaceEvents",
                    &self.r#notify_keyspace_events,
                ),
                to_pulumi_object_field(
                    "rdbBackupEnabled",
                    &self.r#rdb_backup_enabled,
                ),
                to_pulumi_object_field(
                    "rdbBackupFrequency",
                    &self.r#rdb_backup_frequency,
                ),
                to_pulumi_object_field(
                    "rdbBackupMaxSnapshotCount",
                    &self.r#rdb_backup_max_snapshot_count,
                ),
                to_pulumi_object_field(
                    "rdbStorageConnectionString",
                    &self.r#rdb_storage_connection_string,
                ),
                to_pulumi_object_field(
                    "storageAccountSubscriptionId",
                    &self.r#storage_account_subscription_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CacheRedisConfiguration {
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
                        let field_value = match fields_map.get("activeDirectoryAuthenticationEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'activeDirectoryAuthenticationEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aof_backup_enabled: {
                        let field_value = match fields_map.get("aofBackupEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'aofBackupEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aof_storage_connection_string_0: {
                        let field_value = match fields_map.get("aofStorageConnectionString0") {
                            Some(value) => value,
                            None => bail!("Missing field 'aofStorageConnectionString0' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aof_storage_connection_string_1: {
                        let field_value = match fields_map.get("aofStorageConnectionString1") {
                            Some(value) => value,
                            None => bail!("Missing field 'aofStorageConnectionString1' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authentication_enabled: {
                        let field_value = match fields_map.get("authenticationEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'authenticationEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_persistence_authentication_method: {
                        let field_value = match fields_map.get("dataPersistenceAuthenticationMethod") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataPersistenceAuthenticationMethod' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("maxfragmentationmemoryReserved") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxfragmentationmemoryReserved' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maxmemory_delta: {
                        let field_value = match fields_map.get("maxmemoryDelta") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxmemoryDelta' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maxmemory_policy: {
                        let field_value = match fields_map.get("maxmemoryPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxmemoryPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maxmemory_reserved: {
                        let field_value = match fields_map.get("maxmemoryReserved") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxmemoryReserved' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notify_keyspace_events: {
                        let field_value = match fields_map.get("notifyKeyspaceEvents") {
                            Some(value) => value,
                            None => bail!("Missing field 'notifyKeyspaceEvents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rdb_backup_enabled: {
                        let field_value = match fields_map.get("rdbBackupEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'rdbBackupEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rdb_backup_frequency: {
                        let field_value = match fields_map.get("rdbBackupFrequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'rdbBackupFrequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rdb_backup_max_snapshot_count: {
                        let field_value = match fields_map.get("rdbBackupMaxSnapshotCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'rdbBackupMaxSnapshotCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rdb_storage_connection_string: {
                        let field_value = match fields_map.get("rdbStorageConnectionString") {
                            Some(value) => value,
                            None => bail!("Missing field 'rdbStorageConnectionString' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_subscription_id: {
                        let field_value = match fields_map.get("storageAccountSubscriptionId") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageAccountSubscriptionId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
