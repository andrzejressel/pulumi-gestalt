#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MediaInsightsPipelineConfigurationRealTimeAlertConfiguration {
    /// Disables real time alert rules.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
    /// Collection of real time alert rules
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Vec<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRule>,
}
