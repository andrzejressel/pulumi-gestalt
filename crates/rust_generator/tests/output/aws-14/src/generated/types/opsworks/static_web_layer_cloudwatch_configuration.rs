#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StaticWebLayerCloudwatchConfiguration {
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "logStreams")]
    pub r#log_streams: Option<Vec<super::super::types::opsworks::StaticWebLayerCloudwatchConfigurationLogStream>>,
}
