#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VoiceConnectorStreamingMediaInsightsConfiguration {
    /// The media insights configuration that will be invoked by the Voice Connector.
    #[builder(into)]
    #[serde(rename = "configurationArn")]
    pub r#configuration_arn: Option<String>,
    /// When `true`, the media insights configuration is not enabled. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
}
