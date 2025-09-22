#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AiIndexEndpointDeployedIndexPrivateEndpointPscAutomatedEndpoint {
    /// (Output)
    /// ip Address created by the automated forwarding rule.
    #[builder(into)]
    #[serde(rename = "matchAddress")]
    pub r#match_address: Option<String>,
    /// (Output)
    /// Corresponding network in pscAutomationConfigs.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// (Output)
    /// Corresponding projectId in pscAutomationConfigs
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Option<String>,
}
