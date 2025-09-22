#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyVmWorkloadProtectionPolicyRetentionWeekly {
    /// The number of weekly backups to keep. Possible values are between `1` and `5163`.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: i32,
    /// The weekday backups to retain. Possible values are `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` or `Saturday`.
    #[builder(into)]
    #[serde(rename = "weekdays")]
    pub r#weekdays: Vec<String>,
}
