#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterServerlessv2ScalingConfiguration {
    /// Maximum capacity for an Aurora DB cluster in `provisioned` DB engine mode. The maximum capacity must be greater than or equal to the minimum capacity. Valid capacity values are in a range of `0` up to `256` in steps of `0.5`.
    #[builder(into)]
    #[serde(rename = "maxCapacity")]
    pub r#max_capacity: f64,
    /// Minimum capacity for an Aurora DB cluster in `provisioned` DB engine mode. The minimum capacity must be lesser than or equal to the maximum capacity. Valid capacity values are in a range of `0` up to `256` in steps of `0.5`.
    #[builder(into)]
    #[serde(rename = "minCapacity")]
    pub r#min_capacity: f64,
    /// Time, in seconds, before an Aurora DB cluster in `provisioned` DB engine mode is paused. Valid values are `300` through `86400`.
    #[builder(into)]
    #[serde(rename = "secondsUntilAutoPause")]
    pub r#seconds_until_auto_pause: Option<i32>,
}
