#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AssetDiscoveryStatus {
    /// The duration of the last discovery run.
    #[builder(into)]
    #[serde(rename = "lastRunDuration")]
    pub r#last_run_duration: Option<String>,
    /// The start time of the last discovery run.
    #[builder(into)]
    #[serde(rename = "lastRunTime")]
    pub r#last_run_time: Option<String>,
    /// Additional information about the current state.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// Output only. Current state of the asset. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// Data Stats of the asset reported by discovery.
    #[builder(into)]
    #[serde(rename = "stats")]
    pub r#stats: Option<Vec<super::super::types::dataplex::AssetDiscoveryStatusStat>>,
    /// Output only. The time when the asset was last updated.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Option<String>,
}
