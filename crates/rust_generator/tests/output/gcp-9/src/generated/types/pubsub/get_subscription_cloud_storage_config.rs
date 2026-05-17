#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSubscriptionCloudStorageConfig {
    /// If set, message data will be written to Cloud Storage in Avro format.
    #[builder(into)]
    #[serde(rename = "avroConfigs")]
    pub r#avro_configs: Vec<super::super::types::pubsub::GetSubscriptionCloudStorageConfigAvroConfig>,
    /// User-provided name for the Cloud Storage bucket. The bucket must be created by the user. The bucket name must be without any prefix like "gs://".
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: String,
    /// User-provided format string specifying how to represent datetimes in Cloud Storage filenames.
    #[builder(into)]
    #[serde(rename = "filenameDatetimeFormat")]
    pub r#filename_datetime_format: String,
    /// User-provided prefix for Cloud Storage filename.
    #[builder(into)]
    #[serde(rename = "filenamePrefix")]
    pub r#filename_prefix: String,
    /// User-provided suffix for Cloud Storage filename. Must not end in "/".
    #[builder(into)]
    #[serde(rename = "filenameSuffix")]
    pub r#filename_suffix: String,
    /// The maximum bytes that can be written to a Cloud Storage file before a new file is created. Min 1 KB, max 10 GiB.
    /// The maxBytes limit may be exceeded in cases where messages are larger than the limit.
    #[builder(into)]
    #[serde(rename = "maxBytes")]
    pub r#max_bytes: i32,
    /// The maximum duration that can elapse before a new Cloud Storage file is created. Min 1 minute, max 10 minutes, default 5 minutes.
    /// May not exceed the subscription's acknowledgement deadline.
    /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "maxDuration")]
    pub r#max_duration: String,
    /// The maximum messages that can be written to a Cloud Storage file before a new file is created. Min 1000 messages.
    #[builder(into)]
    #[serde(rename = "maxMessages")]
    pub r#max_messages: i32,
    /// The service account to use to write to Cloud Storage. If not specified, the Pub/Sub
    /// [service agent](https://cloud.google.com/iam/docs/service-agents),
    /// service-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com, is used.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: String,
    /// An output-only field that indicates whether or not the subscription can receive messages.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSubscriptionCloudStorageConfig {
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
                "avro_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#avro_configs,
                )
                .await,
            );
            map.insert(
                "bucket".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bucket,
                )
                .await,
            );
            map.insert(
                "filename_datetime_format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filename_datetime_format,
                )
                .await,
            );
            map.insert(
                "filename_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filename_prefix,
                )
                .await,
            );
            map.insert(
                "filename_suffix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filename_suffix,
                )
                .await,
            );
            map.insert(
                "max_bytes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_bytes,
                )
                .await,
            );
            map.insert(
                "max_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_duration,
                )
                .await,
            );
            map.insert(
                "max_messages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_messages,
                )
                .await,
            );
            map.insert(
                "service_account_email".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_account_email,
                )
                .await,
            );
            map.insert(
                "state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#state,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSubscriptionCloudStorageConfig {
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
                    r#avro_configs: {
                        let field_value = match fields_map.get("avro_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'avro_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#filename_datetime_format: {
                        let field_value = match fields_map.get("filename_datetime_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'filename_datetime_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filename_prefix: {
                        let field_value = match fields_map.get("filename_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'filename_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filename_suffix: {
                        let field_value = match fields_map.get("filename_suffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'filename_suffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_bytes: {
                        let field_value = match fields_map.get("max_bytes") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_bytes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_duration: {
                        let field_value = match fields_map.get("max_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_messages: {
                        let field_value = match fields_map.get("max_messages") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_messages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_email: {
                        let field_value = match fields_map.get("service_account_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
