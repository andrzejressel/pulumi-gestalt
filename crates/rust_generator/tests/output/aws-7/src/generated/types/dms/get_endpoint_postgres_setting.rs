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
