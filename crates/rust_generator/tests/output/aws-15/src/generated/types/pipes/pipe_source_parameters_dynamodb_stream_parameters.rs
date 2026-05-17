#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeSourceParametersDynamodbStreamParameters {
    /// The maximum number of records to include in each batch. Maximum value of 10000.
    #[builder(into)]
    #[serde(rename = "batchSize")]
    pub r#batch_size: Option<i32>,
    /// Define the target queue to send dead-letter queue events to. Detailed below.
    #[builder(into)]
    #[serde(rename = "deadLetterConfig")]
    pub r#dead_letter_config: Option<Box<super::super::types::pipes::PipeSourceParametersDynamodbStreamParametersDeadLetterConfig>>,
    /// The maximum length of a time to wait for events. Maximum value of 300.
    #[builder(into)]
    #[serde(rename = "maximumBatchingWindowInSeconds")]
    pub r#maximum_batching_window_in_seconds: Option<i32>,
    /// Discard records older than the specified age. The default value is -1, which sets the maximum age to infinite. When the value is set to infinite, EventBridge never discards old records. Maximum value of 604,800.
    #[builder(into)]
    #[serde(rename = "maximumRecordAgeInSeconds")]
    pub r#maximum_record_age_in_seconds: Option<i32>,
    /// Discard records after the specified number of retries. The default value is -1, which sets the maximum number of retries to infinite. When MaximumRetryAttempts is infinite, EventBridge retries failed records until the record expires in the event source. Maximum value of 10,000.
    #[builder(into)]
    #[serde(rename = "maximumRetryAttempts")]
    pub r#maximum_retry_attempts: Option<i32>,
    /// Define how to handle item process failures. AUTOMATIC_BISECT halves each batch and retry each half until all the records are processed or there is one failed message left in the batch. Valid values: AUTOMATIC_BISECT.
    #[builder(into)]
    #[serde(rename = "onPartialBatchItemFailure")]
    pub r#on_partial_batch_item_failure: Option<String>,
    /// The number of batches to process concurrently from each shard. The default value is 1. Maximum value of 10.
    #[builder(into)]
    #[serde(rename = "parallelizationFactor")]
    pub r#parallelization_factor: Option<i32>,
    /// The position in a stream from which to start reading. Valid values: TRIM_HORIZON, LATEST.
    #[builder(into)]
    #[serde(rename = "startingPosition")]
    pub r#starting_position: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipeSourceParametersDynamodbStreamParameters {
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
                "dead_letter_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dead_letter_config,
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
                "maximum_record_age_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_record_age_in_seconds,
                )
                .await,
            );
            map.insert(
                "maximum_retry_attempts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_retry_attempts,
                )
                .await,
            );
            map.insert(
                "on_partial_batch_item_failure".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#on_partial_batch_item_failure,
                )
                .await,
            );
            map.insert(
                "parallelization_factor".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parallelization_factor,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipeSourceParametersDynamodbStreamParameters {
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
                    r#dead_letter_config: {
                        let field_value = match fields_map.get("dead_letter_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'dead_letter_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#maximum_record_age_in_seconds: {
                        let field_value = match fields_map.get("maximum_record_age_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_record_age_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_retry_attempts: {
                        let field_value = match fields_map.get("maximum_retry_attempts") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_retry_attempts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_partial_batch_item_failure: {
                        let field_value = match fields_map.get("on_partial_batch_item_failure") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_partial_batch_item_failure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parallelization_factor: {
                        let field_value = match fields_map.get("parallelization_factor") {
                            Some(value) => value,
                            None => bail!("Missing field 'parallelization_factor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
