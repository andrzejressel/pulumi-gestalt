#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BackupPlanBackupScheduleRpoConfigExclusionWindowSingleOccurrenceDate {
    /// Day of a month.
    #[builder(into, default)]
    #[serde(rename = "day")]
    pub r#day: Box<Option<i32>>,
    /// Month of a year.
    #[builder(into, default)]
    #[serde(rename = "month")]
    pub r#month: Box<Option<i32>>,
    /// Year of the date.
    #[builder(into, default)]
    #[serde(rename = "year")]
    pub r#year: Box<Option<i32>>,
}
