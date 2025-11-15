#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterNodeConfigContainerdConfigPrivateRegistryAccessConfig {
    /// Parameters for configuring CA certificate and domains.
    #[builder(into)]
    #[serde(rename = "certificateAuthorityDomainConfigs")]
    pub r#certificate_authority_domain_configs: Vec<super::super::types::container::GetClusterNodeConfigContainerdConfigPrivateRegistryAccessConfigCertificateAuthorityDomainConfig>,
    /// Whether or not private registries are configured.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
}
