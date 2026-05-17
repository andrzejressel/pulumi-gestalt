#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfiguration {
    /// Labels all personally identifiable information (PII) identified in Transcript events.
    #[builder(into)]
    #[serde(rename = "contentIdentificationType")]
    pub r#content_identification_type: Option<String>,
    /// Redacts all personally identifiable information (PII) identified in Transcript events.
    #[builder(into)]
    #[serde(rename = "contentRedactionType")]
    pub r#content_redaction_type: Option<String>,
    /// Enables partial result stabilization in Transcript events.
    #[builder(into)]
    #[serde(rename = "enablePartialResultsStabilization")]
    pub r#enable_partial_results_stabilization: Option<bool>,
    /// Filters partial Utterance events from delivery to the insights target.
    #[builder(into)]
    #[serde(rename = "filterPartialResults")]
    pub r#filter_partial_results: Option<bool>,
    /// Language code for the transcription model.
    #[builder(into)]
    #[serde(rename = "languageCode")]
    pub r#language_code: String,
    /// Name of custom language model for transcription.
    #[builder(into)]
    #[serde(rename = "languageModelName")]
    pub r#language_model_name: Option<String>,
    /// Level of stability to use when partial results stabilization is enabled.
    #[builder(into)]
    #[serde(rename = "partialResultsStability")]
    pub r#partial_results_stability: Option<String>,
    /// Types of personally identifiable information (PII) to redact from a Transcript event.
    #[builder(into)]
    #[serde(rename = "piiEntityTypes")]
    pub r#pii_entity_types: Option<String>,
    /// Enables speaker partitioning (diarization) in your Transcript events.
    #[builder(into)]
    #[serde(rename = "showSpeakerLabel")]
    pub r#show_speaker_label: Option<bool>,
    /// Method for applying a vocabulary filter to Transcript events.
    #[builder(into)]
    #[serde(rename = "vocabularyFilterMethod")]
    pub r#vocabulary_filter_method: Option<String>,
    /// Name of the custom vocabulary filter to use when processing Transcript events.
    #[builder(into)]
    #[serde(rename = "vocabularyFilterName")]
    pub r#vocabulary_filter_name: Option<String>,
    /// Name of the custom vocabulary to use when processing Transcript events.
    #[builder(into)]
    #[serde(rename = "vocabularyName")]
    pub r#vocabulary_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfiguration {
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
                    "content_identification_type",
                    &self.r#content_identification_type,
                ),
                to_pulumi_object_field(
                    "content_redaction_type",
                    &self.r#content_redaction_type,
                ),
                to_pulumi_object_field(
                    "enable_partial_results_stabilization",
                    &self.r#enable_partial_results_stabilization,
                ),
                to_pulumi_object_field(
                    "filter_partial_results",
                    &self.r#filter_partial_results,
                ),
                to_pulumi_object_field(
                    "language_code",
                    &self.r#language_code,
                ),
                to_pulumi_object_field(
                    "language_model_name",
                    &self.r#language_model_name,
                ),
                to_pulumi_object_field(
                    "partial_results_stability",
                    &self.r#partial_results_stability,
                ),
                to_pulumi_object_field(
                    "pii_entity_types",
                    &self.r#pii_entity_types,
                ),
                to_pulumi_object_field(
                    "show_speaker_label",
                    &self.r#show_speaker_label,
                ),
                to_pulumi_object_field(
                    "vocabulary_filter_method",
                    &self.r#vocabulary_filter_method,
                ),
                to_pulumi_object_field(
                    "vocabulary_filter_name",
                    &self.r#vocabulary_filter_name,
                ),
                to_pulumi_object_field(
                    "vocabulary_name",
                    &self.r#vocabulary_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfiguration {
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
                    r#content_identification_type: {
                        let field_value = match fields_map.get("content_identification_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_identification_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#content_redaction_type: {
                        let field_value = match fields_map.get("content_redaction_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_redaction_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_partial_results_stabilization: {
                        let field_value = match fields_map.get("enable_partial_results_stabilization") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_partial_results_stabilization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_partial_results: {
                        let field_value = match fields_map.get("filter_partial_results") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter_partial_results' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#language_code: {
                        let field_value = match fields_map.get("language_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'language_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#language_model_name: {
                        let field_value = match fields_map.get("language_model_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'language_model_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#partial_results_stability: {
                        let field_value = match fields_map.get("partial_results_stability") {
                            Some(value) => value,
                            None => bail!("Missing field 'partial_results_stability' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pii_entity_types: {
                        let field_value = match fields_map.get("pii_entity_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'pii_entity_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#show_speaker_label: {
                        let field_value = match fields_map.get("show_speaker_label") {
                            Some(value) => value,
                            None => bail!("Missing field 'show_speaker_label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vocabulary_filter_method: {
                        let field_value = match fields_map.get("vocabulary_filter_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'vocabulary_filter_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vocabulary_filter_name: {
                        let field_value = match fields_map.get("vocabulary_filter_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'vocabulary_filter_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vocabulary_name: {
                        let field_value = match fields_map.get("vocabulary_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'vocabulary_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
