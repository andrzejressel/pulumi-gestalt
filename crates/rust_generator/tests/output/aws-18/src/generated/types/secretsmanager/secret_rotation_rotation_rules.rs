#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecretRotationRotationRules {
    /// Specifies the number of days between automatic scheduled rotations of the secret. Either `automatically_after_days` or `schedule_expression` must be specified.
    #[builder(into)]
    #[serde(rename = "automaticallyAfterDays")]
    pub r#automatically_after_days: Option<i32>,
    /// The length of the rotation window in hours. For example, `3h` for a three hour window.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Option<String>,
    /// A `cron()` or `rate()` expression that defines the schedule for rotating your secret. Either `automatically_after_days` or `schedule_expression` must be specified.
    #[builder(into)]
    #[serde(rename = "scheduleExpression")]
    pub r#schedule_expression: Option<String>,
}
