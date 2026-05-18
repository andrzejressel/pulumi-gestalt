#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomLayerCloudwatchConfigurationLogStream {
    /// Specifies the max number of log events in a batch, up to `10000`. The default value is `1000`.
    #[builder(into)]
    #[serde(rename = "batchCount")]
    pub r#batch_count: Option<i32>,
    /// Specifies the maximum size of log events in a batch, in bytes, up to `1048576` bytes. The default value is `32768` bytes.
    #[builder(into)]
    #[serde(rename = "batchSize")]
    pub r#batch_size: Option<i32>,
    /// Specifies the time duration for the batching of log events. The minimum value is `5000` and default value is `5000`.
    #[builder(into)]
    #[serde(rename = "bufferDuration")]
    pub r#buffer_duration: Option<i32>,
    /// Specifies how the timestamp is extracted from logs. For more information, see the CloudWatch Logs Agent Reference (https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/AgentReference.html).
    #[builder(into)]
    #[serde(rename = "datetimeFormat")]
    pub r#datetime_format: Option<String>,
    /// Specifies the encoding of the log file so that the file can be read correctly. The default is `utf_8`.
    #[builder(into)]
    #[serde(rename = "encoding")]
    pub r#encoding: Option<String>,
    /// Specifies log files that you want to push to CloudWatch Logs. File can point to a specific file or multiple files (by using wild card characters such as /var/log/system.log*).
    #[builder(into)]
    #[serde(rename = "file")]
    pub r#file: String,
    /// Specifies the range of lines for identifying a file. The valid values are one number, or two dash-delimited numbers, such as `1`, `2-5`. The default value is `1`.
    #[builder(into)]
    #[serde(rename = "fileFingerprintLines")]
    pub r#file_fingerprint_lines: Option<String>,
    /// Specifies where to start to read data (`start_of_file` or `end_of_file`). The default is `start_of_file`.
    #[builder(into)]
    #[serde(rename = "initialPosition")]
    pub r#initial_position: Option<String>,
    /// Specifies the destination log group. A log group is created automatically if it doesn't already exist.
    #[builder(into)]
    #[serde(rename = "logGroupName")]
    pub r#log_group_name: String,
    /// Specifies the pattern for identifying the start of a log message.
    #[builder(into)]
    #[serde(rename = "multilineStartPattern")]
    pub r#multiline_start_pattern: Option<String>,
    /// Specifies the time zone of log event time stamps.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CustomLayerCloudwatchConfigurationLogStream {
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
                    "batch_count",
                    &self.r#batch_count,
                ),
                to_pulumi_object_field(
                    "batch_size",
                    &self.r#batch_size,
                ),
                to_pulumi_object_field(
                    "buffer_duration",
                    &self.r#buffer_duration,
                ),
                to_pulumi_object_field(
                    "datetime_format",
                    &self.r#datetime_format,
                ),
                to_pulumi_object_field(
                    "encoding",
                    &self.r#encoding,
                ),
                to_pulumi_object_field(
                    "file",
                    &self.r#file,
                ),
                to_pulumi_object_field(
                    "file_fingerprint_lines",
                    &self.r#file_fingerprint_lines,
                ),
                to_pulumi_object_field(
                    "initial_position",
                    &self.r#initial_position,
                ),
                to_pulumi_object_field(
                    "log_group_name",
                    &self.r#log_group_name,
                ),
                to_pulumi_object_field(
                    "multiline_start_pattern",
                    &self.r#multiline_start_pattern,
                ),
                to_pulumi_object_field(
                    "time_zone",
                    &self.r#time_zone,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CustomLayerCloudwatchConfigurationLogStream {
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
