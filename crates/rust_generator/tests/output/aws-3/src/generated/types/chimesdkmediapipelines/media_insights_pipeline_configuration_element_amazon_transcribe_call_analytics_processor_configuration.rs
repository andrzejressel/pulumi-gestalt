#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfiguration {
    /// Filter for category events to be delivered to insights target.
    #[builder(into)]
    #[serde(rename = "callAnalyticsStreamCategories")]
    pub r#call_analytics_stream_categories: Option<Vec<String>>,
    /// Labels all personally identifiable information (PII) identified in Utterance events.
    #[builder(into)]
    #[serde(rename = "contentIdentificationType")]
    pub r#content_identification_type: Option<String>,
    /// Redacts all personally identifiable information (PII) identified in Utterance events.
    #[builder(into)]
    #[serde(rename = "contentRedactionType")]
    pub r#content_redaction_type: Option<String>,
    /// Enables partial result stabilization in Utterance events.
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
    /// Types of personally identifiable information (PII) to redact from an Utterance event.
    #[builder(into)]
    #[serde(rename = "piiEntityTypes")]
    pub r#pii_entity_types: Option<String>,
    /// Settings for post call analytics.
    #[builder(into)]
    #[serde(rename = "postCallAnalyticsSettings")]
    pub r#post_call_analytics_settings: Option<Box<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettings>>,
    /// Method for applying a vocabulary filter to Utterance events.
    #[builder(into)]
    #[serde(rename = "vocabularyFilterMethod")]
    pub r#vocabulary_filter_method: Option<String>,
    /// Name of the custom vocabulary filter to use when processing Utterance events.
    #[builder(into)]
    #[serde(rename = "vocabularyFilterName")]
    pub r#vocabulary_filter_name: Option<String>,
    /// Name of the custom vocabulary to use when processing Utterance events.
    #[builder(into)]
    #[serde(rename = "vocabularyName")]
    pub r#vocabulary_name: Option<String>,
}
