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
