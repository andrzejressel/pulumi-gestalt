#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NextGenerationFirewallVirtualHubPanoramaDestinationNat {
    #[builder(into)]
    #[serde(rename = "backendConfig")]
    pub r#backend_config: Option<Box<super::super::types::paloalto::NextGenerationFirewallVirtualHubPanoramaDestinationNatBackendConfig>>,
    #[builder(into)]
    #[serde(rename = "frontendConfig")]
    pub r#frontend_config: Option<Box<super::super::types::paloalto::NextGenerationFirewallVirtualHubPanoramaDestinationNatFrontendConfig>>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
}
