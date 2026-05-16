#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyTlsCertificate {
    /// The ID of the Key Vault, where the secret or certificate is stored.
    #[builder(into)]
    #[serde(rename = "keyVaultSecretId")]
    pub r#key_vault_secret_id: String,
    /// The name of the certificate.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
