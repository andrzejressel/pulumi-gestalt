#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDatabaseInstanceReplicaConfiguration {
    /// PEM representation of the trusted CA's x509 certificate.
    #[builder(into)]
    #[serde(rename = "caCertificate")]
    pub r#ca_certificate: String,
    /// Specifies if a SQL Server replica is a cascadable replica. A cascadable replica is a SQL Server cross region replica that supports replica(s) under it.
    #[builder(into)]
    #[serde(rename = "cascadableReplica")]
    pub r#cascadable_replica: bool,
    /// PEM representation of the replica's x509 certificate.
    #[builder(into)]
    #[serde(rename = "clientCertificate")]
    pub r#client_certificate: String,
    /// PEM representation of the replica's private key. The corresponding public key in encoded in the client_certificate.
    #[builder(into)]
    #[serde(rename = "clientKey")]
    pub r#client_key: String,
    /// The number of seconds between connect retries. MySQL's default is 60 seconds.
    #[builder(into)]
    #[serde(rename = "connectRetryInterval")]
    pub r#connect_retry_interval: i32,
    /// Path to a SQL file in Google Cloud Storage from which replica instances are created. Format is gs://bucket/filename.
    #[builder(into)]
    #[serde(rename = "dumpFilePath")]
    pub r#dump_file_path: String,
    /// Specifies if the replica is the failover target. If the field is set to true the replica will be designated as a failover replica. If the master instance fails, the replica instance will be promoted as the new master instance. Not supported for Postgres
    #[builder(into)]
    #[serde(rename = "failoverTarget")]
    pub r#failover_target: bool,
    /// Time in ms between replication heartbeats.
    #[builder(into)]
    #[serde(rename = "masterHeartbeatPeriod")]
    pub r#master_heartbeat_period: i32,
    /// Password for the replication connection.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// Permissible ciphers for use in SSL encryption.
    #[builder(into)]
    #[serde(rename = "sslCipher")]
    pub r#ssl_cipher: String,
    /// Username for replication connection.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: String,
    /// True if the master's common name value is checked during the SSL handshake.
    #[builder(into)]
    #[serde(rename = "verifyServerCertificate")]
    pub r#verify_server_certificate: bool,
}
