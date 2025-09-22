#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ContactsRotationRecurrenceMonthlySetting {
    /// (Required) The day of the month when monthly recurring on-call rotations begin.
    #[builder(into)]
    #[serde(rename = "dayOfMonth")]
    pub r#day_of_month: i32,
    /// (Required) The hand off time. See Hand Off Time for more details.
    #[builder(into)]
    #[serde(rename = "handOffTime")]
    pub r#hand_off_time: Option<Box<super::super::types::ssm::ContactsRotationRecurrenceMonthlySettingHandOffTime>>,
}
