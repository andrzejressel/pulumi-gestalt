#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AssetResourceStatus {
    /// Additional information about the current state.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// Output only. Current state of the asset. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// Output only. The time when the asset was last updated.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Option<String>,
}
