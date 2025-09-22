#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
