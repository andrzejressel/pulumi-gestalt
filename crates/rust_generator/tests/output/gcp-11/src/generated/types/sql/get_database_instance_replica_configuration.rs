#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDatabaseInstanceReplicaConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "ca_certificate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ca_certificate,
                )
                .await,
            );
            map.insert(
                "cascadable_replica".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cascadable_replica,
                )
                .await,
            );
            map.insert(
                "client_certificate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_certificate,
                )
                .await,
            );
            map.insert(
                "client_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_key,
                )
                .await,
            );
            map.insert(
                "connect_retry_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connect_retry_interval,
                )
                .await,
            );
            map.insert(
                "dump_file_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dump_file_path,
                )
                .await,
            );
            map.insert(
                "failover_target".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#failover_target,
                )
                .await,
            );
            map.insert(
                "master_heartbeat_period".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#master_heartbeat_period,
                )
                .await,
            );
            map.insert(
                "password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#password,
                )
                .await,
            );
            map.insert(
                "ssl_cipher".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssl_cipher,
                )
                .await,
            );
            map.insert(
                "username".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#username,
                )
                .await,
            );
            map.insert(
                "verify_server_certificate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#verify_server_certificate,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDatabaseInstanceReplicaConfiguration {
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
