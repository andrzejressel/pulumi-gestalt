#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkWatcherFlowLogTrafficAnalytics {
    /// Boolean flag to enable/disable traffic analytics.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// How frequently service should do flow analytics in minutes. Defaults to `60`.
    #[builder(into)]
    #[serde(rename = "intervalInMinutes")]
    pub r#interval_in_minutes: Option<i32>,
    /// The resource GUID of the attached workspace.
    #[builder(into)]
    #[serde(rename = "workspaceId")]
    pub r#workspace_id: String,
    /// The location of the attached workspace.
    #[builder(into)]
    #[serde(rename = "workspaceRegion")]
    pub r#workspace_region: String,
    /// The resource ID of the attached workspace.
    #[builder(into)]
    #[serde(rename = "workspaceResourceId")]
    pub r#workspace_resource_id: String,
}
