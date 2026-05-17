#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatabaseInstanceReplicaConfiguration {
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
                    "ca_certificate",
                    &self.r#ca_certificate,
                ),
                to_pulumi_object_field(
                    "cascadable_replica",
                    &self.r#cascadable_replica,
                ),
                to_pulumi_object_field(
                    "client_certificate",
                    &self.r#client_certificate,
                ),
                to_pulumi_object_field(
                    "client_key",
                    &self.r#client_key,
                ),
                to_pulumi_object_field(
                    "connect_retry_interval",
                    &self.r#connect_retry_interval,
                ),
                to_pulumi_object_field(
                    "dump_file_path",
                    &self.r#dump_file_path,
                ),
                to_pulumi_object_field(
                    "failover_target",
                    &self.r#failover_target,
                ),
                to_pulumi_object_field(
                    "master_heartbeat_period",
                    &self.r#master_heartbeat_period,
                ),
                to_pulumi_object_field(
                    "password",
                    &self.r#password,
                ),
                to_pulumi_object_field(
                    "ssl_cipher",
                    &self.r#ssl_cipher,
                ),
                to_pulumi_object_field(
                    "username",
                    &self.r#username,
                ),
                to_pulumi_object_field(
                    "verify_server_certificate",
                    &self.r#verify_server_certificate,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatabaseInstanceReplicaConfiguration {
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
                    r#ca_certificate: {
                        let field_value = match fields_map.get("ca_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'ca_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cascadable_replica: {
                        let field_value = match fields_map.get("cascadable_replica") {
                            Some(value) => value,
                            None => bail!("Missing field 'cascadable_replica' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_certificate: {
                        let field_value = match fields_map.get("client_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_key: {
                        let field_value = match fields_map.get("client_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connect_retry_interval: {
                        let field_value = match fields_map.get("connect_retry_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'connect_retry_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dump_file_path: {
                        let field_value = match fields_map.get("dump_file_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'dump_file_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failover_target: {
                        let field_value = match fields_map.get("failover_target") {
                            Some(value) => value,
                            None => bail!("Missing field 'failover_target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#master_heartbeat_period: {
                        let field_value = match fields_map.get("master_heartbeat_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'master_heartbeat_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#password: {
                        let field_value = match fields_map.get("password") {
                            Some(value) => value,
                            None => bail!("Missing field 'password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_cipher: {
                        let field_value = match fields_map.get("ssl_cipher") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_cipher' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#username: {
                        let field_value = match fields_map.get("username") {
                            Some(value) => value,
                            None => bail!("Missing field 'username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#verify_server_certificate: {
                        let field_value = match fields_map.get("verify_server_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'verify_server_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
