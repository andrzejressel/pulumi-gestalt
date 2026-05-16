#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountThrottleSetting {
    /// Absolute maximum number of times API Gateway allows the API to be called per second (RPS).
    #[builder(into)]
    #[serde(rename = "burstLimit")]
    pub r#burst_limit: f64,
    /// Number of times API Gateway allows the API to be called per second on average (RPS).
    #[builder(into)]
    #[serde(rename = "rateLimit")]
    pub r#rate_limit: f64,
}
