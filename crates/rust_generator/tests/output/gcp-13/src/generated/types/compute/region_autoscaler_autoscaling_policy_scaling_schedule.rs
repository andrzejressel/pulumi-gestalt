#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionAutoscalerAutoscalingPolicyScalingSchedule {
    /// An optional description of this resource.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// A boolean value that specifies if a scaling schedule can influence autoscaler recommendations. If set to true, then a scaling schedule has no effect.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
    /// The duration of time intervals (in seconds) for which this scaling schedule will be running. The minimum allowed value is 300.
    #[builder(into)]
    #[serde(rename = "durationSec")]
    pub r#duration_sec: i32,
    /// Minimum number of VM instances that autoscaler will recommend in time intervals starting according to schedule.
    #[builder(into)]
    #[serde(rename = "minRequiredReplicas")]
    pub r#min_required_replicas: i32,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The start timestamps of time intervals when this scaling schedule should provide a scaling signal. This field uses the extended cron format (with an optional year field).
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: String,
    /// The time zone to be used when interpreting the schedule. The value of this field must be a time zone name from the tz database: http://en.wikipedia.org/wiki/Tz_database.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Option<String>,
}
