#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RiskBehaviorBehavior {
    /// Whether this risk behavior type is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Name of this risk behavior type
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Risk level. Available values: `low`, `medium`, `high`
    #[builder(into)]
    #[serde(rename = "riskLevel")]
    pub r#risk_level: String,
}
