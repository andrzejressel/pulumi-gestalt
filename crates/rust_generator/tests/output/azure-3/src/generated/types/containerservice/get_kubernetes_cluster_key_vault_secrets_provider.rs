#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetKubernetesClusterKeyVaultSecretsProvider {
    /// A `secret_identity` block as documented below.
    #[builder(into)]
    #[serde(rename = "secretIdentities")]
    pub r#secret_identities: Vec<super::super::types::containerservice::GetKubernetesClusterKeyVaultSecretsProviderSecretIdentity>,
    /// Is secret rotation enabled?
    #[builder(into)]
    #[serde(rename = "secretRotationEnabled")]
    pub r#secret_rotation_enabled: bool,
    /// The interval to poll for secret rotation.
    #[builder(into)]
    #[serde(rename = "secretRotationInterval")]
    pub r#secret_rotation_interval: String,
}
