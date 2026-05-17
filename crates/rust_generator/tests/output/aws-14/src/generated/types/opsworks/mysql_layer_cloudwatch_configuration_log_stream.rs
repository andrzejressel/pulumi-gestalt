#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MysqlLayerCloudwatchConfigurationLogStream {
    #[builder(into)]
    #[serde(rename = "batchCount")]
    pub r#batch_count: Option<i32>,
    #[builder(into)]
    #[serde(rename = "batchSize")]
    pub r#batch_size: Option<i32>,
    #[builder(into)]
    #[serde(rename = "bufferDuration")]
    pub r#buffer_duration: Option<i32>,
    #[builder(into)]
    #[serde(rename = "datetimeFormat")]
    pub r#datetime_format: Option<String>,
    #[builder(into)]
    #[serde(rename = "encoding")]
    pub r#encoding: Option<String>,
    #[builder(into)]
    #[serde(rename = "file")]
    pub r#file: String,
    #[builder(into)]
    #[serde(rename = "fileFingerprintLines")]
    pub r#file_fingerprint_lines: Option<String>,
    #[builder(into)]
    #[serde(rename = "initialPosition")]
    pub r#initial_position: Option<String>,
    #[builder(into)]
    #[serde(rename = "logGroupName")]
    pub r#log_group_name: String,
    #[builder(into)]
    #[serde(rename = "multilineStartPattern")]
    pub r#multiline_start_pattern: Option<String>,
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MysqlLayerCloudwatchConfigurationLogStream {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "batch_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#batch_count,
                )
                .await,
            );
            map.insert(
                "batch_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#batch_size,
                )
                .await,
            );
            map.insert(
                "buffer_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#buffer_duration,
                )
                .await,
            );
            map.insert(
                "datetime_format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#datetime_format,
                )
                .await,
            );
            map.insert(
                "encoding".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encoding,
                )
                .await,
            );
            map.insert(
                "file".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file,
                )
                .await,
            );
            map.insert(
                "file_fingerprint_lines".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_fingerprint_lines,
                )
                .await,
            );
            map.insert(
                "initial_position".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#initial_position,
                )
                .await,
            );
            map.insert(
                "log_group_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_group_name,
                )
                .await,
            );
            map.insert(
                "multiline_start_pattern".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#multiline_start_pattern,
                )
                .await,
            );
            map.insert(
                "time_zone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_zone,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MysqlLayerCloudwatchConfigurationLogStream {
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
                    r#batch_count: {
                        let field_value = match fields_map.get("batch_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'batch_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#batch_size: {
                        let field_value = match fields_map.get("batch_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'batch_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#buffer_duration: {
                        let field_value = match fields_map.get("buffer_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'buffer_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#datetime_format: {
                        let field_value = match fields_map.get("datetime_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'datetime_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encoding: {
                        let field_value = match fields_map.get("encoding") {
                            Some(value) => value,
                            None => bail!("Missing field 'encoding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file: {
                        let field_value = match fields_map.get("file") {
                            Some(value) => value,
                            None => bail!("Missing field 'file' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_fingerprint_lines: {
                        let field_value = match fields_map.get("file_fingerprint_lines") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_fingerprint_lines' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#initial_position: {
                        let field_value = match fields_map.get("initial_position") {
                            Some(value) => value,
                            None => bail!("Missing field 'initial_position' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_group_name: {
                        let field_value = match fields_map.get("log_group_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_group_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multiline_start_pattern: {
                        let field_value = match fields_map.get("multiline_start_pattern") {
                            Some(value) => value,
                            None => bail!("Missing field 'multiline_start_pattern' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_zone: {
                        let field_value = match fields_map.get("time_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
