#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UsagePlanThrottleSettings {
    #[builder(into)]
    #[serde(rename = "burstLimit")]
    pub r#burst_limit: Option<i32>,
    #[builder(into)]
    #[serde(rename = "rateLimit")]
    pub r#rate_limit: Option<f64>,
}
