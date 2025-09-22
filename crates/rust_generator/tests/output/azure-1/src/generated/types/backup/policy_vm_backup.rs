#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyVmBackup {
    /// Sets the backup frequency. Possible values are `Hourly`, `Daily` and `Weekly`.
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: String,
    /// Duration of the backup window in hours. Possible values are between `4` and `24` This is used when `frequency` is `Hourly`.
    /// 
    /// > **NOTE:** `hour_duration` must be multiplier of `hour_interval`
    #[builder(into)]
    #[serde(rename = "hourDuration")]
    pub r#hour_duration: Option<i32>,
    /// Interval in hour at which backup is triggered. Possible values are `4`, `6`, `8` and `12`. This is used when `frequency` is `Hourly`.
    #[builder(into)]
    #[serde(rename = "hourInterval")]
    pub r#hour_interval: Option<i32>,
    /// The time of day to perform the backup in 24hour format.
    #[builder(into)]
    #[serde(rename = "time")]
    pub r#time: String,
    /// The days of the week to perform backups on. Must be one of `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` or `Saturday`. This is used when `frequency` is `Weekly`.
    #[builder(into)]
    #[serde(rename = "weekdays")]
    pub r#weekdays: Option<Vec<String>>,
}
