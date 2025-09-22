#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BackupPlanBackupScheduleRpoConfigExclusionWindowStartTime {
    /// Hours of day in 24 hour format.
    #[builder(into)]
    #[serde(rename = "hours")]
    pub r#hours: Option<i32>,
    /// Minutes of hour of day.
    #[builder(into)]
    #[serde(rename = "minutes")]
    pub r#minutes: Option<i32>,
    /// Fractions of seconds in nanoseconds.
    #[builder(into)]
    #[serde(rename = "nanos")]
    pub r#nanos: Option<i32>,
    /// Seconds of minutes of the time.
    #[builder(into)]
    #[serde(rename = "seconds")]
    pub r#seconds: Option<i32>,
}
