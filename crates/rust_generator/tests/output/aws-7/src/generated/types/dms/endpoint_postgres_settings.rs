#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointPostgresSettings {
    /// For use with change data capture (CDC) only, this attribute has AWS DMS bypass foreign keys and user triggers to reduce the time it takes to bulk load data.
    #[builder(into)]
    #[serde(rename = "afterConnectScript")]
    pub r#after_connect_script: Option<String>,
    /// The Babelfish for Aurora PostgreSQL database name for the endpoint.
    #[builder(into)]
    #[serde(rename = "babelfishDatabaseName")]
    pub r#babelfish_database_name: Option<String>,
    /// To capture DDL events, AWS DMS creates various artifacts in the PostgreSQL database when the task starts.
    #[builder(into)]
    #[serde(rename = "captureDdls")]
    pub r#capture_ddls: Option<bool>,
    /// Specifies the default behavior of the replication's handling of PostgreSQL- compatible endpoints that require some additional configuration, such as Babelfish endpoints.
    #[builder(into)]
    #[serde(rename = "databaseMode")]
    pub r#database_mode: Option<String>,
    /// Sets the schema in which the operational DDL database artifacts are created. Default is `public`.
    #[builder(into)]
    #[serde(rename = "ddlArtifactsSchema")]
    pub r#ddl_artifacts_schema: Option<String>,
    /// Sets the client statement timeout for the PostgreSQL instance, in seconds. Default value is `60`.
    #[builder(into)]
    #[serde(rename = "executeTimeout")]
    pub r#execute_timeout: Option<i32>,
    /// When set to `true`, this value causes a task to fail if the actual size of a LOB column is greater than the specified `LobMaxSize`. Default is `false`.
    #[builder(into)]
    #[serde(rename = "failTasksOnLobTruncation")]
    pub r#fail_tasks_on_lob_truncation: Option<bool>,
    /// The write-ahead log (WAL) heartbeat feature mimics a dummy transaction. By doing this, it prevents idle logical replication slots from holding onto old WAL logs, which can result in storage full situations on the source.
    #[builder(into)]
    #[serde(rename = "heartbeatEnable")]
    pub r#heartbeat_enable: Option<bool>,
    /// Sets the WAL heartbeat frequency (in minutes). Default value is `5`.
    #[builder(into)]
    #[serde(rename = "heartbeatFrequency")]
    pub r#heartbeat_frequency: Option<i32>,
    /// Sets the schema in which the heartbeat artifacts are created. Default value is `public`.
    #[builder(into)]
    #[serde(rename = "heartbeatSchema")]
    pub r#heartbeat_schema: Option<String>,
    /// You can use PostgreSQL endpoint settings to map a boolean as a boolean from your PostgreSQL source to a Amazon Redshift target. Default value is `false`.
    #[builder(into)]
    #[serde(rename = "mapBooleanAsBoolean")]
    pub r#map_boolean_as_boolean: Option<bool>,
    /// Optional When true, DMS migrates JSONB values as CLOB.
    #[builder(into)]
    #[serde(rename = "mapJsonbAsClob")]
    pub r#map_jsonb_as_clob: Option<bool>,
    /// Optional When true, DMS migrates LONG values as VARCHAR.
    #[builder(into)]
    #[serde(rename = "mapLongVarcharAs")]
    pub r#map_long_varchar_as: Option<String>,
    /// Specifies the maximum size (in KB) of any .csv file used to transfer data to PostgreSQL. Default is `32,768 KB`.
    #[builder(into)]
    #[serde(rename = "maxFileSize")]
    pub r#max_file_size: Option<i32>,
    /// Specifies the plugin to use to create a replication slot. Valid values: `pglogical`, `test_decoding`.
    #[builder(into)]
    #[serde(rename = "pluginName")]
    pub r#plugin_name: Option<String>,
    /// Sets the name of a previously created logical replication slot for a CDC load of the PostgreSQL source instance.
    #[builder(into)]
    #[serde(rename = "slotName")]
    pub r#slot_name: Option<String>,
}
