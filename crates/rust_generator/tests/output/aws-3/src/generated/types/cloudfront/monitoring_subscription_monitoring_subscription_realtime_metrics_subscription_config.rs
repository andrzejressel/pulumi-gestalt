#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MonitoringSubscriptionMonitoringSubscriptionRealtimeMetricsSubscriptionConfig {
    /// A flag that indicates whether additional CloudWatch metrics are enabled for a given CloudFront distribution. Valid values are `Enabled` and `Disabled`. See below.
    #[builder(into)]
    #[serde(rename = "realtimeMetricsSubscriptionStatus")]
    pub r#realtime_metrics_subscription_status: String,
}
