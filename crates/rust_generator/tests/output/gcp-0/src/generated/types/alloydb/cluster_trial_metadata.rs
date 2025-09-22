#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterTrialMetadata {
    /// End time of the trial cluster.
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: Option<String>,
    /// Grace end time of the trial cluster.
    #[builder(into)]
    #[serde(rename = "graceEndTime")]
    pub r#grace_end_time: Option<String>,
    /// Start time of the trial cluster.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
    /// Upgrade time of the trial cluster to standard cluster.
    #[builder(into)]
    #[serde(rename = "upgradeTime")]
    pub r#upgrade_time: Option<String>,
}
