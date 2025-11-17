#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionSecurityPolicyRuleRateLimitOptionsBanThreshold {
    /// Number of HTTP(S) requests for calculating the threshold.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Option<i32>,
    /// Interval over which the threshold is computed.
    #[builder(into)]
    #[serde(rename = "intervalSec")]
    pub r#interval_sec: Option<i32>,
}
