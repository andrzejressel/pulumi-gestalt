#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EnvironmentConfigMasterAuthorizedNetworksConfig {
    /// cidr_blocks define up to 50 external networks that could access Kubernetes master through HTTPS.
    #[builder(into)]
    #[serde(rename = "cidrBlocks")]
    pub r#cidr_blocks: Option<Vec<super::super::types::composer::EnvironmentConfigMasterAuthorizedNetworksConfigCidrBlock>>,
    /// Whether or not master authorized networks is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
}
