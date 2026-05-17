#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInstanceStorageConfigStorageConfig {
    /// A block that specifies the configuration of the Kinesis Firehose delivery stream. Documented below.
    #[builder(into)]
    #[serde(rename = "kinesisFirehoseConfigs")]
    pub r#kinesis_firehose_configs: Vec<super::super::types::connect::GetInstanceStorageConfigStorageConfigKinesisFirehoseConfig>,
    /// A block that specifies the configuration of the Kinesis data stream. Documented below.
    #[builder(into)]
    #[serde(rename = "kinesisStreamConfigs")]
    pub r#kinesis_stream_configs: Vec<super::super::types::connect::GetInstanceStorageConfigStorageConfigKinesisStreamConfig>,
    /// A block that specifies the configuration of the Kinesis video stream. Documented below.
    #[builder(into)]
    #[serde(rename = "kinesisVideoStreamConfigs")]
    pub r#kinesis_video_stream_configs: Vec<super::super::types::connect::GetInstanceStorageConfigStorageConfigKinesisVideoStreamConfig>,
    /// A block that specifies the configuration of S3 Bucket. Documented below.
    #[builder(into)]
    #[serde(rename = "s3Configs")]
    pub r#s_3_configs: Vec<super::super::types::connect::GetInstanceStorageConfigStorageConfigS3Config>,
    /// A valid storage type. Valid Values: `S3` | `KINESIS_VIDEO_STREAM` | `KINESIS_STREAM` | `KINESIS_FIREHOSE`.
    #[builder(into)]
    #[serde(rename = "storageType")]
    pub r#storage_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetInstanceStorageConfigStorageConfig {
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
                "kinesis_firehose_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kinesis_firehose_configs,
                )
                .await,
            );
            map.insert(
                "kinesis_stream_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kinesis_stream_configs,
                )
                .await,
            );
            map.insert(
                "kinesis_video_stream_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kinesis_video_stream_configs,
                )
                .await,
            );
            map.insert(
                "s_3_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_configs,
                )
                .await,
            );
            map.insert(
                "storage_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetInstanceStorageConfigStorageConfig {
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
                    r#kinesis_firehose_configs: {
                        let field_value = match fields_map.get("kinesis_firehose_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_firehose_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_stream_configs: {
                        let field_value = match fields_map.get("kinesis_stream_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_stream_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_video_stream_configs: {
                        let field_value = match fields_map.get("kinesis_video_stream_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_video_stream_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_configs: {
                        let field_value = match fields_map.get("s_3_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_type: {
                        let field_value = match fields_map.get("storage_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
