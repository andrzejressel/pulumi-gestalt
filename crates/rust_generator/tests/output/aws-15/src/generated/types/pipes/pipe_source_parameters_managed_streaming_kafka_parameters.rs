#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeSourceParametersManagedStreamingKafkaParameters {
    /// The maximum number of records to include in each batch. Maximum value of 10000.
    #[builder(into)]
    #[serde(rename = "batchSize")]
    pub r#batch_size: Option<i32>,
    /// The name of the destination queue to consume. Maximum value of 200.
    #[builder(into)]
    #[serde(rename = "consumerGroupId")]
    pub r#consumer_group_id: Option<String>,
    /// The credentials needed to access the resource. Detailed below.
    #[builder(into)]
    #[serde(rename = "credentials")]
    pub r#credentials: Option<Box<super::super::types::pipes::PipeSourceParametersManagedStreamingKafkaParametersCredentials>>,
    /// The maximum length of a time to wait for events. Maximum value of 300.
    #[builder(into)]
    #[serde(rename = "maximumBatchingWindowInSeconds")]
    pub r#maximum_batching_window_in_seconds: Option<i32>,
    /// The position in a stream from which to start reading. Valid values: TRIM_HORIZON, LATEST.
    #[builder(into)]
    #[serde(rename = "startingPosition")]
    pub r#starting_position: Option<String>,
    /// The name of the topic that the pipe will read from. Maximum length of 249.
    #[builder(into)]
    #[serde(rename = "topicName")]
    pub r#topic_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipeSourceParametersManagedStreamingKafkaParameters {
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
                "batch_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#batch_size,
                )
                .await,
            );
            map.insert(
                "consumer_group_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#consumer_group_id,
                )
                .await,
            );
            map.insert(
                "credentials".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#credentials,
                )
                .await,
            );
            map.insert(
                "maximum_batching_window_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_batching_window_in_seconds,
                )
                .await,
            );
            map.insert(
                "starting_position".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#starting_position,
                )
                .await,
            );
            map.insert(
                "topic_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#topic_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipeSourceParametersManagedStreamingKafkaParameters {
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
                    r#batch_size: {
                        let field_value = match fields_map.get("batch_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'batch_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#consumer_group_id: {
                        let field_value = match fields_map.get("consumer_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'consumer_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#credentials: {
                        let field_value = match fields_map.get("credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_batching_window_in_seconds: {
                        let field_value = match fields_map.get("maximum_batching_window_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_batching_window_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#starting_position: {
                        let field_value = match fields_map.get("starting_position") {
                            Some(value) => value,
                            None => bail!("Missing field 'starting_position' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topic_name: {
                        let field_value = match fields_map.get("topic_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'topic_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
