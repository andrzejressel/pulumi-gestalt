#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkConnectionMonitorTestConfigurationSuccessThreshold {
    /// The maximum percentage of failed checks permitted for a test to be successful.
    #[builder(into)]
    #[serde(rename = "checksFailedPercent")]
    pub r#checks_failed_percent: Option<i32>,
    /// The maximum round-trip time in milliseconds permitted for a test to be successful.
    #[builder(into)]
    #[serde(rename = "roundTripTimeMs")]
    pub r#round_trip_time_ms: Option<f64>,
}
