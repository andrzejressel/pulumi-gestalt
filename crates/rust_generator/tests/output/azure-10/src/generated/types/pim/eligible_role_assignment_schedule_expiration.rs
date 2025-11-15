#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EligibleRoleAssignmentScheduleExpiration {
    /// The duration of the role assignment in days. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "durationDays")]
    pub r#duration_days: Option<i32>,
    /// The duration of the role assignment in hours. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "durationHours")]
    pub r#duration_hours: Option<i32>,
    /// The end date/time of the role assignment. Changing this forces a new resource to be created.
    /// 
    /// > Note: Only one of `duration_days`, `duration_hours` or `end_date_time` should be specified.
    #[builder(into)]
    #[serde(rename = "endDateTime")]
    pub r#end_date_time: Option<String>,
}
