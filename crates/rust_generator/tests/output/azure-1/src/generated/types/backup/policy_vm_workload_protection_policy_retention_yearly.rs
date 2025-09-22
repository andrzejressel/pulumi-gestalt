#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyVmWorkloadProtectionPolicyRetentionYearly {
    /// The number of yearly backups to keep. Possible values are between `1` and `99`
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: i32,
    /// The retention schedule format type for yearly retention policy. Possible values are `Daily` and `Weekly`.
    #[builder(into)]
    #[serde(rename = "formatType")]
    pub r#format_type: String,
    /// The monthday backups to retain. Possible values are between `0` and `28`.
    #[builder(into)]
    #[serde(rename = "monthdays")]
    pub r#monthdays: Option<Vec<i32>>,
    /// The months of the year to retain backups of. Possible values are `January`, `February`, `March`, `April`, `May`, `June`, `July`, `August`, `September`, `October`, `November` and `December`.
    #[builder(into)]
    #[serde(rename = "months")]
    pub r#months: Vec<String>,
    /// The weekday backups to retain. Possible values are `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` or `Saturday`.
    #[builder(into)]
    #[serde(rename = "weekdays")]
    pub r#weekdays: Option<Vec<String>>,
    /// The weeks of the month to retain backups of. Possible values are `First`, `Second`, `Third`, `Fourth`, `Last`.
    #[builder(into)]
    #[serde(rename = "weeks")]
    pub r#weeks: Option<Vec<String>>,
}
