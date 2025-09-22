#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatabaseInstanceReplicaConfiguration {
    /// PEM representation of the trusted CA's x509
    /// certificate.
    #[builder(into)]
    #[serde(rename = "caCertificate")]
    pub r#ca_certificate: Option<String>,
    /// Specifies if the replica is a cascadable replica. If true, instance must be in different region from primary.
    /// 
    /// > **NOTE:** Only supported for SQL Server database.
    #[builder(into)]
    #[serde(rename = "cascadableReplica")]
    pub r#cascadable_replica: Option<bool>,
    /// PEM representation of the replica's x509
    /// certificate.
    #[builder(into)]
    #[serde(rename = "clientCertificate")]
    pub r#client_certificate: Option<String>,
    /// PEM representation of the replica's private key. The
    /// corresponding public key in encoded in the `client_certificate`.
    #[builder(into)]
    #[serde(rename = "clientKey")]
    pub r#client_key: Option<String>,
    /// The number of seconds
    /// between connect retries. MySQL's default is 60 seconds.
    #[builder(into)]
    #[serde(rename = "connectRetryInterval")]
    pub r#connect_retry_interval: Option<i32>,
    /// Path to a SQL file in GCS from which replica
    /// instances are created. Format is `gs://bucket/filename`. Note, if the master
    /// instance is a source representation instance this field must be present.
    #[builder(into)]
    #[serde(rename = "dumpFilePath")]
    pub r#dump_file_path: Option<String>,
    /// Specifies if the replica is the failover target.
    /// If the field is set to true the replica will be designated as a failover replica.
    /// If the master instance fails, the replica instance will be promoted as
    /// the new master instance.
    /// > **NOTE:** Not supported for Postgres database.
    #[builder(into)]
    #[serde(rename = "failoverTarget")]
    pub r#failover_target: Option<bool>,
    /// Time in ms between replication
    /// heartbeats.
    #[builder(into)]
    #[serde(rename = "masterHeartbeatPeriod")]
    pub r#master_heartbeat_period: Option<i32>,
    /// Password for the replication connection.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// Permissible ciphers for use in SSL encryption.
    #[builder(into)]
    #[serde(rename = "sslCipher")]
    pub r#ssl_cipher: Option<String>,
    /// Username for replication connection.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
    /// True if the master's common name
    /// value is checked during the SSL handshake.
    #[builder(into)]
    #[serde(rename = "verifyServerCertificate")]
    pub r#verify_server_certificate: Option<bool>,
}
