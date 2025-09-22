#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterOmsAgent {
    /// The ID of the Log Analytics Workspace which the OMS Agent should send data to.
    #[builder(into)]
    #[serde(rename = "logAnalyticsWorkspaceId")]
    pub r#log_analytics_workspace_id: String,
    /// Is managed identity authentication for monitoring enabled?
    #[builder(into)]
    #[serde(rename = "msiAuthForMonitoringEnabled")]
    pub r#msi_auth_for_monitoring_enabled: Option<bool>,
    /// An `oms_agent_identity` block is exported. The exported attributes are defined below.
    #[builder(into)]
    #[serde(rename = "omsAgentIdentities")]
    pub r#oms_agent_identities: Option<Vec<super::super::types::containerservice::KubernetesClusterOmsAgentOmsAgentIdentity>>,
}
