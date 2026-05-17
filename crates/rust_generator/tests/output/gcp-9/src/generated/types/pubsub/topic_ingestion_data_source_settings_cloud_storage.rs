#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TopicIngestionDataSourceSettingsCloudStorage {
    /// Configuration for reading Cloud Storage data in Avro binary format. The
    /// bytes of each object will be set to the `data` field of a Pub/Sub message.
    #[builder(into)]
    #[serde(rename = "avroFormat")]
    pub r#avro_format: Option<Box<super::super::types::pubsub::TopicIngestionDataSourceSettingsCloudStorageAvroFormat>>,
    /// Cloud Storage bucket. The bucket name must be without any
    /// prefix like "gs://". See the bucket naming requirements:
    /// https://cloud.google.com/storage/docs/buckets#naming.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: String,
    /// Glob pattern used to match objects that will be ingested. If unset, all
    /// objects will be ingested. See the supported patterns:
    /// https://cloud.google.com/storage/docs/json_api/v1/objects/list#list-objects-and-prefixes-using-glob
    #[builder(into)]
    #[serde(rename = "matchGlob")]
    pub r#match_glob: Option<String>,
    /// The timestamp set in RFC3339 text format. If set, only objects with a
    /// larger or equal timestamp will be ingested. Unset by default, meaning
    /// all objects will be ingested.
    #[builder(into)]
    #[serde(rename = "minimumObjectCreateTime")]
    pub r#minimum_object_create_time: Option<String>,
    /// Configuration for reading Cloud Storage data written via Cloud Storage
    /// subscriptions(See https://cloud.google.com/pubsub/docs/cloudstorage). The
    /// data and attributes fields of the originally exported Pub/Sub message
    /// will be restored when publishing.
    #[builder(into)]
    #[serde(rename = "pubsubAvroFormat")]
    pub r#pubsub_avro_format: Option<Box<super::super::types::pubsub::TopicIngestionDataSourceSettingsCloudStoragePubsubAvroFormat>>,
    /// Configuration for reading Cloud Storage data in text format. Each line of
    /// text as specified by the delimiter will be set to the `data` field of a
    /// Pub/Sub message.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "textFormat")]
    pub r#text_format: Option<Box<super::super::types::pubsub::TopicIngestionDataSourceSettingsCloudStorageTextFormat>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TopicIngestionDataSourceSettingsCloudStorage {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "avro_format",
                    &self.r#avro_format,
                ),
                to_pulumi_object_field(
                    "bucket",
                    &self.r#bucket,
                ),
                to_pulumi_object_field(
                    "match_glob",
                    &self.r#match_glob,
                ),
                to_pulumi_object_field(
                    "minimum_object_create_time",
                    &self.r#minimum_object_create_time,
                ),
                to_pulumi_object_field(
                    "pubsub_avro_format",
                    &self.r#pubsub_avro_format,
                ),
                to_pulumi_object_field(
                    "text_format",
                    &self.r#text_format,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TopicIngestionDataSourceSettingsCloudStorage {
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
                    r#avro_format: {
                        let field_value = match fields_map.get("avro_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'avro_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket: {
                        let field_value = match fields_map.get("bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#match_glob: {
                        let field_value = match fields_map.get("match_glob") {
                            Some(value) => value,
                            None => bail!("Missing field 'match_glob' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_object_create_time: {
                        let field_value = match fields_map.get("minimum_object_create_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_object_create_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pubsub_avro_format: {
                        let field_value = match fields_map.get("pubsub_avro_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'pubsub_avro_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#text_format: {
                        let field_value = match fields_map.get("text_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
