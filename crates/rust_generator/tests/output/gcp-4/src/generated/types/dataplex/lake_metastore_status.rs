#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LakeMetastoreStatus {
    /// The URI of the endpoint used to access the Metastore service.
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: Option<String>,
    /// Additional information about the current status.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// Output only. Current state of the lake. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// Output only. The time when the lake was last updated.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Option<String>,
}
