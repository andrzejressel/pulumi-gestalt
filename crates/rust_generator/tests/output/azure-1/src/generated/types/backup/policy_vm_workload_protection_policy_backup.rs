#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyVmWorkloadProtectionPolicyBackup {
    /// The backup frequency for the VM Workload Backup Policy. Possible values are `Daily` and `Weekly`.
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: Option<String>,
    /// The backup frequency in minutes for the VM Workload Backup Policy. Possible values are `15`, `30`, `60`, `120`, `240`, `480`, `720` and `1440`.
    #[builder(into)]
    #[serde(rename = "frequencyInMinutes")]
    pub r#frequency_in_minutes: Option<i32>,
    /// The time of day to perform the backup in 24hour format.
    #[builder(into)]
    #[serde(rename = "time")]
    pub r#time: Option<String>,
    /// The days of the week to perform backups on. Possible values are `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` or `Saturday`. This is used when `frequency` is `Weekly`.
    #[builder(into)]
    #[serde(rename = "weekdays")]
    pub r#weekdays: Option<Vec<String>>,
}
