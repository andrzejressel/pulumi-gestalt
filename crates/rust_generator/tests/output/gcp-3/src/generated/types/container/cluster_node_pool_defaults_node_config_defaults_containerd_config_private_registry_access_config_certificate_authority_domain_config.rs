#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNodePoolDefaultsNodeConfigDefaultsContainerdConfigPrivateRegistryAccessConfigCertificateAuthorityDomainConfig {
    /// List of fully-qualified-domain-names. IPv4s and port specification are supported.
    #[builder(into)]
    #[serde(rename = "fqdns")]
    pub r#fqdns: Vec<String>,
    /// Parameters for configuring a certificate hosted in GCP SecretManager.
    #[builder(into)]
    #[serde(rename = "gcpSecretManagerCertificateConfig")]
    pub r#gcp_secret_manager_certificate_config: Box<super::super::types::container::ClusterNodePoolDefaultsNodeConfigDefaultsContainerdConfigPrivateRegistryAccessConfigCertificateAuthorityDomainConfigGcpSecretManagerCertificateConfig>,
}
