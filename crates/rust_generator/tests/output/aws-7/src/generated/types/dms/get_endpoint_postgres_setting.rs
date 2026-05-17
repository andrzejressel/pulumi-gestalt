#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEndpointPostgresSetting {
    #[builder(into)]
    #[serde(rename = "afterConnectScript")]
    pub r#after_connect_script: String,
    #[builder(into)]
    #[serde(rename = "babelfishDatabaseName")]
    pub r#babelfish_database_name: String,
    #[builder(into)]
    #[serde(rename = "captureDdls")]
    pub r#capture_ddls: bool,
    #[builder(into)]
    #[serde(rename = "databaseMode")]
    pub r#database_mode: String,
    #[builder(into)]
    #[serde(rename = "ddlArtifactsSchema")]
    pub r#ddl_artifacts_schema: String,
    #[builder(into)]
    #[serde(rename = "executeTimeout")]
    pub r#execute_timeout: i32,
    #[builder(into)]
    #[serde(rename = "failTasksOnLobTruncation")]
    pub r#fail_tasks_on_lob_truncation: bool,
    #[builder(into)]
    #[serde(rename = "heartbeatEnable")]
    pub r#heartbeat_enable: bool,
    #[builder(into)]
    #[serde(rename = "heartbeatFrequency")]
    pub r#heartbeat_frequency: i32,
    #[builder(into)]
    #[serde(rename = "heartbeatSchema")]
    pub r#heartbeat_schema: String,
    #[builder(into)]
    #[serde(rename = "mapBooleanAsBoolean")]
    pub r#map_boolean_as_boolean: bool,
    #[builder(into)]
    #[serde(rename = "mapJsonbAsClob")]
    pub r#map_jsonb_as_clob: bool,
    #[builder(into)]
    #[serde(rename = "mapLongVarcharAs")]
    pub r#map_long_varchar_as: String,
    #[builder(into)]
    #[serde(rename = "maxFileSize")]
    pub r#max_file_size: i32,
    #[builder(into)]
    #[serde(rename = "pluginName")]
    pub r#plugin_name: String,
    #[builder(into)]
    #[serde(rename = "slotName")]
    pub r#slot_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetEndpointPostgresSetting {
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
                    "after_connect_script",
                    &self.r#after_connect_script,
                ),
                to_pulumi_object_field(
                    "babelfish_database_name",
                    &self.r#babelfish_database_name,
                ),
                to_pulumi_object_field(
                    "capture_ddls",
                    &self.r#capture_ddls,
                ),
                to_pulumi_object_field(
                    "database_mode",
                    &self.r#database_mode,
                ),
                to_pulumi_object_field(
                    "ddl_artifacts_schema",
                    &self.r#ddl_artifacts_schema,
                ),
                to_pulumi_object_field(
                    "execute_timeout",
                    &self.r#execute_timeout,
                ),
                to_pulumi_object_field(
                    "fail_tasks_on_lob_truncation",
                    &self.r#fail_tasks_on_lob_truncation,
                ),
                to_pulumi_object_field(
                    "heartbeat_enable",
                    &self.r#heartbeat_enable,
                ),
                to_pulumi_object_field(
                    "heartbeat_frequency",
                    &self.r#heartbeat_frequency,
                ),
                to_pulumi_object_field(
                    "heartbeat_schema",
                    &self.r#heartbeat_schema,
                ),
                to_pulumi_object_field(
                    "map_boolean_as_boolean",
                    &self.r#map_boolean_as_boolean,
                ),
                to_pulumi_object_field(
                    "map_jsonb_as_clob",
                    &self.r#map_jsonb_as_clob,
                ),
                to_pulumi_object_field(
                    "map_long_varchar_as",
                    &self.r#map_long_varchar_as,
                ),
                to_pulumi_object_field(
                    "max_file_size",
                    &self.r#max_file_size,
                ),
                to_pulumi_object_field(
                    "plugin_name",
                    &self.r#plugin_name,
                ),
                to_pulumi_object_field(
                    "slot_name",
                    &self.r#slot_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetEndpointPostgresSetting {
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
