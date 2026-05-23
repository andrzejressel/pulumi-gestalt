#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfiguration {
    /// Filter for category events to be delivered to insights target.
    #[builder(into)]
    pub r#call_analytics_stream_categories: Option<Vec<String>>,
    /// Labels all personally identifiable information (PII) identified in Utterance events.
    #[builder(into)]
    pub r#content_identification_type: Option<String>,
    /// Redacts all personally identifiable information (PII) identified in Utterance events.
    #[builder(into)]
    pub r#content_redaction_type: Option<String>,
    /// Enables partial result stabilization in Utterance events.
    #[builder(into)]
    pub r#enable_partial_results_stabilization: Option<bool>,
    /// Filters partial Utterance events from delivery to the insights target.
    #[builder(into)]
    pub r#filter_partial_results: Option<bool>,
    /// Language code for the transcription model.
    #[builder(into)]
    pub r#language_code: String,
    /// Name of custom language model for transcription.
    #[builder(into)]
    pub r#language_model_name: Option<String>,
    /// Level of stability to use when partial results stabilization is enabled.
    #[builder(into)]
    pub r#partial_results_stability: Option<String>,
    /// Types of personally identifiable information (PII) to redact from an Utterance event.
    #[builder(into)]
    pub r#pii_entity_types: Option<String>,
    /// Settings for post call analytics.
    #[builder(into)]
    pub r#post_call_analytics_settings: Option<Box<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettings>>,
    /// Method for applying a vocabulary filter to Utterance events.
    #[builder(into)]
    pub r#vocabulary_filter_method: Option<String>,
    /// Name of the custom vocabulary filter to use when processing Utterance events.
    #[builder(into)]
    pub r#vocabulary_filter_name: Option<String>,
    /// Name of the custom vocabulary to use when processing Utterance events.
    #[builder(into)]
    pub r#vocabulary_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfiguration {
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
                    "callAnalyticsStreamCategories",
                    &self.r#call_analytics_stream_categories,
                ),
                to_pulumi_object_field(
                    "contentIdentificationType",
                    &self.r#content_identification_type,
                ),
                to_pulumi_object_field(
                    "contentRedactionType",
                    &self.r#content_redaction_type,
                ),
                to_pulumi_object_field(
                    "enablePartialResultsStabilization",
                    &self.r#enable_partial_results_stabilization,
                ),
                to_pulumi_object_field(
                    "filterPartialResults",
                    &self.r#filter_partial_results,
                ),
                to_pulumi_object_field(
                    "languageCode",
                    &self.r#language_code,
                ),
                to_pulumi_object_field(
                    "languageModelName",
                    &self.r#language_model_name,
                ),
                to_pulumi_object_field(
                    "partialResultsStability",
                    &self.r#partial_results_stability,
                ),
                to_pulumi_object_field(
                    "piiEntityTypes",
                    &self.r#pii_entity_types,
                ),
                to_pulumi_object_field(
                    "postCallAnalyticsSettings",
                    &self.r#post_call_analytics_settings,
                ),
                to_pulumi_object_field(
                    "vocabularyFilterMethod",
                    &self.r#vocabulary_filter_method,
                ),
                to_pulumi_object_field(
                    "vocabularyFilterName",
                    &self.r#vocabulary_filter_name,
                ),
                to_pulumi_object_field(
                    "vocabularyName",
                    &self.r#vocabulary_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfiguration {
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
                    r#call_analytics_stream_categories: {
                        let field_value = match fields_map.get("callAnalyticsStreamCategories") {
                            Some(value) => value,
                            None => bail!("Missing field 'callAnalyticsStreamCategories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#content_identification_type: {
                        let field_value = match fields_map.get("contentIdentificationType") {
                            Some(value) => value,
                            None => bail!("Missing field 'contentIdentificationType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#content_redaction_type: {
                        let field_value = match fields_map.get("contentRedactionType") {
                            Some(value) => value,
                            None => bail!("Missing field 'contentRedactionType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_partial_results_stabilization: {
                        let field_value = match fields_map.get("enablePartialResultsStabilization") {
                            Some(value) => value,
                            None => bail!("Missing field 'enablePartialResultsStabilization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_partial_results: {
                        let field_value = match fields_map.get("filterPartialResults") {
                            Some(value) => value,
                            None => bail!("Missing field 'filterPartialResults' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#language_code: {
                        let field_value = match fields_map.get("languageCode") {
                            Some(value) => value,
                            None => bail!("Missing field 'languageCode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#language_model_name: {
                        let field_value = match fields_map.get("languageModelName") {
                            Some(value) => value,
                            None => bail!("Missing field 'languageModelName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#partial_results_stability: {
                        let field_value = match fields_map.get("partialResultsStability") {
                            Some(value) => value,
                            None => bail!("Missing field 'partialResultsStability' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pii_entity_types: {
                        let field_value = match fields_map.get("piiEntityTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'piiEntityTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#post_call_analytics_settings: {
                        let field_value = match fields_map.get("postCallAnalyticsSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'postCallAnalyticsSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vocabulary_filter_method: {
                        let field_value = match fields_map.get("vocabularyFilterMethod") {
                            Some(value) => value,
                            None => bail!("Missing field 'vocabularyFilterMethod' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vocabulary_filter_name: {
                        let field_value = match fields_map.get("vocabularyFilterName") {
                            Some(value) => value,
                            None => bail!("Missing field 'vocabularyFilterName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vocabulary_name: {
                        let field_value = match fields_map.get("vocabularyName") {
                            Some(value) => value,
                            None => bail!("Missing field 'vocabularyName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
