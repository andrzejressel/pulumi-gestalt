#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterMaintenanceWindowAutoUpgrade {
    /// The day of the month for the maintenance run. Required in combination with AbsoluteMonthly frequency. Value between 0 and 31 (inclusive).
    #[builder(into)]
    #[serde(rename = "dayOfMonth")]
    pub r#day_of_month: Option<i32>,
    /// The day of the week for the maintenance run. Required in combination with weekly frequency. Possible values are `Friday`, `Monday`, `Saturday`, `Sunday`, `Thursday`, `Tuesday` and `Wednesday`.
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Option<String>,
    /// The duration of the window for maintenance to run in hours. Possible options are between `4` to `24`.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: i32,
    /// Frequency of maintenance. Possible options are `Weekly`, `AbsoluteMonthly` and `RelativeMonthly`.
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: String,
    /// The interval for maintenance runs. Depending on the frequency this interval is week or month based.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: i32,
    /// One or more `not_allowed` block as defined below.
    #[builder(into)]
    #[serde(rename = "notAlloweds")]
    pub r#not_alloweds: Option<Vec<super::super::types::containerservice::KubernetesClusterMaintenanceWindowAutoUpgradeNotAllowed>>,
    /// The date on which the maintenance window begins to take effect.
    #[builder(into)]
    #[serde(rename = "startDate")]
    pub r#start_date: Option<String>,
    /// The time for maintenance to begin, based on the timezone determined by `utc_offset`. Format is `HH:mm`.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
    /// Used to determine the timezone for cluster maintenance.
    #[builder(into)]
    #[serde(rename = "utcOffset")]
    pub r#utc_offset: Option<String>,
    /// Specifies on which instance of the allowed days specified in `day_of_week` the maintenance occurs. Options are `First`, `Second`, `Third`, `Fourth`, and `Last`.
    /// Required in combination with relative monthly frequency.
    #[builder(into)]
    #[serde(rename = "weekIndex")]
    pub r#week_index: Option<String>,
}
