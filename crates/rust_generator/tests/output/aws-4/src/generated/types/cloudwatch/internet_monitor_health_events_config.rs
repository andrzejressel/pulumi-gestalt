#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InternetMonitorHealthEventsConfig {
    /// The health event threshold percentage set for availability scores.
    #[builder(into)]
    #[serde(rename = "availabilityScoreThreshold")]
    pub r#availability_score_threshold: Option<f64>,
    /// The health event threshold percentage set for performance scores.
    #[builder(into)]
    #[serde(rename = "performanceScoreThreshold")]
    pub r#performance_score_threshold: Option<f64>,
}
