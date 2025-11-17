#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResourcePolicyInstanceSchedulePolicy {
    /// The expiration time of the schedule. The timestamp is an RFC3339 string.
    #[builder(into)]
    #[serde(rename = "expirationTime")]
    pub r#expiration_time: Option<String>,
    /// The start time of the schedule. The timestamp is an RFC3339 string.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
    /// Specifies the time zone to be used in interpreting the schedule. The value of this field must be a time zone name
    /// from the tz database: http://en.wikipedia.org/wiki/Tz_database.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: String,
    /// Specifies the schedule for starting instances.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vmStartSchedule")]
    pub r#vm_start_schedule: Option<Box<super::super::types::compute::ResourcePolicyInstanceSchedulePolicyVmStartSchedule>>,
    /// Specifies the schedule for stopping instances.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vmStopSchedule")]
    pub r#vm_stop_schedule: Option<Box<super::super::types::compute::ResourcePolicyInstanceSchedulePolicyVmStopSchedule>>,
}
