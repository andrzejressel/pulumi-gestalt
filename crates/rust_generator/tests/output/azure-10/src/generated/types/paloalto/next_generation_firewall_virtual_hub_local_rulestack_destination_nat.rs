#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NextGenerationFirewallVirtualHubLocalRulestackDestinationNat {
    #[builder(into)]
    #[serde(rename = "backendConfig")]
    pub r#backend_config: Box<Option<super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackDestinationNatBackendConfig>>,
    #[builder(into)]
    #[serde(rename = "frontendConfig")]
    pub r#frontend_config: Box<Option<super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackDestinationNatFrontendConfig>>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
}
