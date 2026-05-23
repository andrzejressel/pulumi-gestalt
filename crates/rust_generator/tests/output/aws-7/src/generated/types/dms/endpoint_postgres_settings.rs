#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointPostgresSettings {
    /// For use with change data capture (CDC) only, this attribute has AWS DMS bypass foreign keys and user triggers to reduce the time it takes to bulk load data.
    #[builder(into)]
    pub r#after_connect_script: Option<String>,
    /// The Babelfish for Aurora PostgreSQL database name for the endpoint.
    #[builder(into)]
    pub r#babelfish_database_name: Option<String>,
    /// To capture DDL events, AWS DMS creates various artifacts in the PostgreSQL database when the task starts.
    #[builder(into)]
    pub r#capture_ddls: Option<bool>,
    /// Specifies the default behavior of the replication's handling of PostgreSQL- compatible endpoints that require some additional configuration, such as Babelfish endpoints.
    #[builder(into)]
    pub r#database_mode: Option<String>,
    /// Sets the schema in which the operational DDL database artifacts are created. Default is `public`.
    #[builder(into)]
    pub r#ddl_artifacts_schema: Option<String>,
    /// Sets the client statement timeout for the PostgreSQL instance, in seconds. Default value is `60`.
    #[builder(into)]
    pub r#execute_timeout: Option<i32>,
    /// When set to `true`, this value causes a task to fail if the actual size of a LOB column is greater than the specified `LobMaxSize`. Default is `false`.
    #[builder(into)]
    pub r#fail_tasks_on_lob_truncation: Option<bool>,
    /// The write-ahead log (WAL) heartbeat feature mimics a dummy transaction. By doing this, it prevents idle logical replication slots from holding onto old WAL logs, which can result in storage full situations on the source.
    #[builder(into)]
    pub r#heartbeat_enable: Option<bool>,
    /// Sets the WAL heartbeat frequency (in minutes). Default value is `5`.
    #[builder(into)]
    pub r#heartbeat_frequency: Option<i32>,
    /// Sets the schema in which the heartbeat artifacts are created. Default value is `public`.
    #[builder(into)]
    pub r#heartbeat_schema: Option<String>,
    /// You can use PostgreSQL endpoint settings to map a boolean as a boolean from your PostgreSQL source to a Amazon Redshift target. Default value is `false`.
    #[builder(into)]
    pub r#map_boolean_as_boolean: Option<bool>,
    /// Optional When true, DMS migrates JSONB values as CLOB.
    #[builder(into)]
    pub r#map_jsonb_as_clob: Option<bool>,
    /// Optional When true, DMS migrates LONG values as VARCHAR.
    #[builder(into)]
    pub r#map_long_varchar_as: Option<String>,
    /// Specifies the maximum size (in KB) of any .csv file used to transfer data to PostgreSQL. Default is `32,768 KB`.
    #[builder(into)]
    pub r#max_file_size: Option<i32>,
    /// Specifies the plugin to use to create a replication slot. Valid values: `pglogical`, `test_decoding`.
    #[builder(into)]
    pub r#plugin_name: Option<String>,
    /// Sets the name of a previously created logical replication slot for a CDC load of the PostgreSQL source instance.
    #[builder(into)]
    pub r#slot_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointPostgresSettings {
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
                    "afterConnectScript",
                    &self.r#after_connect_script,
                ),
                to_pulumi_object_field(
                    "babelfishDatabaseName",
                    &self.r#babelfish_database_name,
                ),
                to_pulumi_object_field(
                    "captureDdls",
                    &self.r#capture_ddls,
                ),
                to_pulumi_object_field(
                    "databaseMode",
                    &self.r#database_mode,
                ),
                to_pulumi_object_field(
                    "ddlArtifactsSchema",
                    &self.r#ddl_artifacts_schema,
                ),
                to_pulumi_object_field(
                    "executeTimeout",
                    &self.r#execute_timeout,
                ),
                to_pulumi_object_field(
                    "failTasksOnLobTruncation",
                    &self.r#fail_tasks_on_lob_truncation,
                ),
                to_pulumi_object_field(
                    "heartbeatEnable",
                    &self.r#heartbeat_enable,
                ),
                to_pulumi_object_field(
                    "heartbeatFrequency",
                    &self.r#heartbeat_frequency,
                ),
                to_pulumi_object_field(
                    "heartbeatSchema",
                    &self.r#heartbeat_schema,
                ),
                to_pulumi_object_field(
                    "mapBooleanAsBoolean",
                    &self.r#map_boolean_as_boolean,
                ),
                to_pulumi_object_field(
                    "mapJsonbAsClob",
                    &self.r#map_jsonb_as_clob,
                ),
                to_pulumi_object_field(
                    "mapLongVarcharAs",
                    &self.r#map_long_varchar_as,
                ),
                to_pulumi_object_field(
                    "maxFileSize",
                    &self.r#max_file_size,
                ),
                to_pulumi_object_field(
                    "pluginName",
                    &self.r#plugin_name,
                ),
                to_pulumi_object_field(
                    "slotName",
                    &self.r#slot_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("afterConnectScript") {
                            Some(value) => value,
                            None => bail!("Missing field 'afterConnectScript' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#babelfish_database_name: {
                        let field_value = match fields_map.get("babelfishDatabaseName") {
                            Some(value) => value,
                            None => bail!("Missing field 'babelfishDatabaseName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#capture_ddls: {
                        let field_value = match fields_map.get("captureDdls") {
                            Some(value) => value,
                            None => bail!("Missing field 'captureDdls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_mode: {
                        let field_value = match fields_map.get("databaseMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'databaseMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ddl_artifacts_schema: {
                        let field_value = match fields_map.get("ddlArtifactsSchema") {
                            Some(value) => value,
                            None => bail!("Missing field 'ddlArtifactsSchema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execute_timeout: {
                        let field_value = match fields_map.get("executeTimeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'executeTimeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fail_tasks_on_lob_truncation: {
                        let field_value = match fields_map.get("failTasksOnLobTruncation") {
                            Some(value) => value,
                            None => bail!("Missing field 'failTasksOnLobTruncation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#heartbeat_enable: {
                        let field_value = match fields_map.get("heartbeatEnable") {
                            Some(value) => value,
                            None => bail!("Missing field 'heartbeatEnable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#heartbeat_frequency: {
                        let field_value = match fields_map.get("heartbeatFrequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'heartbeatFrequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#heartbeat_schema: {
                        let field_value = match fields_map.get("heartbeatSchema") {
                            Some(value) => value,
                            None => bail!("Missing field 'heartbeatSchema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#map_boolean_as_boolean: {
                        let field_value = match fields_map.get("mapBooleanAsBoolean") {
                            Some(value) => value,
                            None => bail!("Missing field 'mapBooleanAsBoolean' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#map_jsonb_as_clob: {
                        let field_value = match fields_map.get("mapJsonbAsClob") {
                            Some(value) => value,
                            None => bail!("Missing field 'mapJsonbAsClob' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#map_long_varchar_as: {
                        let field_value = match fields_map.get("mapLongVarcharAs") {
                            Some(value) => value,
                            None => bail!("Missing field 'mapLongVarcharAs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_file_size: {
                        let field_value = match fields_map.get("maxFileSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxFileSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#plugin_name: {
                        let field_value = match fields_map.get("pluginName") {
                            Some(value) => value,
                            None => bail!("Missing field 'pluginName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slot_name: {
                        let field_value = match fields_map.get("slotName") {
                            Some(value) => value,
                            None => bail!("Missing field 'slotName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
