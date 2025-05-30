#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterMicrosoftDefender {
    /// Specifies the ID of the Log Analytics Workspace where the audit logs collected by Microsoft Defender should be sent to.
    #[builder(into)]
    #[serde(rename = "logAnalyticsWorkspaceId")]
    pub r#log_analytics_workspace_id: Box<String>,
}
