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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointPostgresSettings {
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
                "after_connect_script".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#after_connect_script,
                )
                .await,
            );
            map.insert(
                "babelfish_database_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#babelfish_database_name,
                )
                .await,
            );
            map.insert(
                "capture_ddls".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#capture_ddls,
                )
                .await,
            );
            map.insert(
                "database_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database_mode,
                )
                .await,
            );
            map.insert(
                "ddl_artifacts_schema".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ddl_artifacts_schema,
                )
                .await,
            );
            map.insert(
                "execute_timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#execute_timeout,
                )
                .await,
            );
            map.insert(
                "fail_tasks_on_lob_truncation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fail_tasks_on_lob_truncation,
                )
                .await,
            );
            map.insert(
                "heartbeat_enable".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#heartbeat_enable,
                )
                .await,
            );
            map.insert(
                "heartbeat_frequency".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#heartbeat_frequency,
                )
                .await,
            );
            map.insert(
                "heartbeat_schema".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#heartbeat_schema,
                )
                .await,
            );
            map.insert(
                "map_boolean_as_boolean".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#map_boolean_as_boolean,
                )
                .await,
            );
            map.insert(
                "map_jsonb_as_clob".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#map_jsonb_as_clob,
                )
                .await,
            );
            map.insert(
                "map_long_varchar_as".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#map_long_varchar_as,
                )
                .await,
            );
            map.insert(
                "max_file_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_file_size,
                )
                .await,
            );
            map.insert(
                "plugin_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#plugin_name,
                )
                .await,
            );
            map.insert(
                "slot_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#slot_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointPostgresSettings {
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
                    r#after_connect_script: {
                        let field_value = match fields_map.get("after_connect_script") {
                            Some(value) => value,
                            None => bail!("Missing field 'after_connect_script' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#babelfish_database_name: {
                        let field_value = match fields_map.get("babelfish_database_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'babelfish_database_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#capture_ddls: {
                        let field_value = match fields_map.get("capture_ddls") {
                            Some(value) => value,
                            None => bail!("Missing field 'capture_ddls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_mode: {
                        let field_value = match fields_map.get("database_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ddl_artifacts_schema: {
                        let field_value = match fields_map.get("ddl_artifacts_schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'ddl_artifacts_schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execute_timeout: {
                        let field_value = match fields_map.get("execute_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'execute_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fail_tasks_on_lob_truncation: {
                        let field_value = match fields_map.get("fail_tasks_on_lob_truncation") {
                            Some(value) => value,
                            None => bail!("Missing field 'fail_tasks_on_lob_truncation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#heartbeat_enable: {
                        let field_value = match fields_map.get("heartbeat_enable") {
                            Some(value) => value,
                            None => bail!("Missing field 'heartbeat_enable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#heartbeat_frequency: {
                        let field_value = match fields_map.get("heartbeat_frequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'heartbeat_frequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#heartbeat_schema: {
                        let field_value = match fields_map.get("heartbeat_schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'heartbeat_schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#map_boolean_as_boolean: {
                        let field_value = match fields_map.get("map_boolean_as_boolean") {
                            Some(value) => value,
                            None => bail!("Missing field 'map_boolean_as_boolean' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#map_jsonb_as_clob: {
                        let field_value = match fields_map.get("map_jsonb_as_clob") {
                            Some(value) => value,
                            None => bail!("Missing field 'map_jsonb_as_clob' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#map_long_varchar_as: {
                        let field_value = match fields_map.get("map_long_varchar_as") {
                            Some(value) => value,
                            None => bail!("Missing field 'map_long_varchar_as' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_file_size: {
                        let field_value = match fields_map.get("max_file_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_file_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#plugin_name: {
                        let field_value = match fields_map.get("plugin_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'plugin_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slot_name: {
                        let field_value = match fields_map.get("slot_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'slot_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
