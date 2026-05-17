#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamDestinationConfigGcsDestinationConfig {
    /// AVRO file format configuration.
    #[builder(into)]
    #[serde(rename = "avroFileFormat")]
    pub r#avro_file_format: Option<Box<super::super::types::datastream::StreamDestinationConfigGcsDestinationConfigAvroFileFormat>>,
    /// The maximum duration for which new events are added before a file is closed and a new file is created.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s". Defaults to 900s.
    #[builder(into)]
    #[serde(rename = "fileRotationInterval")]
    pub r#file_rotation_interval: Option<String>,
    /// The maximum file size to be saved in the bucket.
    #[builder(into)]
    #[serde(rename = "fileRotationMb")]
    pub r#file_rotation_mb: Option<i32>,
    /// JSON file format configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "jsonFileFormat")]
    pub r#json_file_format: Option<Box<super::super::types::datastream::StreamDestinationConfigGcsDestinationConfigJsonFileFormat>>,
    /// Path inside the Cloud Storage bucket to write data to.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StreamDestinationConfigGcsDestinationConfig {
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
                    "avro_file_format",
                    &self.r#avro_file_format,
                ),
                to_pulumi_object_field(
                    "file_rotation_interval",
                    &self.r#file_rotation_interval,
                ),
                to_pulumi_object_field(
                    "file_rotation_mb",
                    &self.r#file_rotation_mb,
                ),
                to_pulumi_object_field(
                    "json_file_format",
                    &self.r#json_file_format,
                ),
                to_pulumi_object_field(
                    "path",
                    &self.r#path,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StreamDestinationConfigGcsDestinationConfig {
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
                    r#avro_file_format: {
                        let field_value = match fields_map.get("avro_file_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'avro_file_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_rotation_interval: {
                        let field_value = match fields_map.get("file_rotation_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_rotation_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_rotation_mb: {
                        let field_value = match fields_map.get("file_rotation_mb") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_rotation_mb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#json_file_format: {
                        let field_value = match fields_map.get("json_file_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'json_file_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
