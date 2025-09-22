#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateOrderCertificate {
    /// The name of the App Service Certificate.
    #[builder(into)]
    #[serde(rename = "certificateName")]
    pub r#certificate_name: Option<String>,
    /// Key Vault resource Id.
    #[builder(into)]
    #[serde(rename = "keyVaultId")]
    pub r#key_vault_id: Option<String>,
    /// Key Vault secret name.
    #[builder(into)]
    #[serde(rename = "keyVaultSecretName")]
    pub r#key_vault_secret_name: Option<String>,
    /// Status of the Key Vault secret.
    #[builder(into)]
    #[serde(rename = "provisioningState")]
    pub r#provisioning_state: Option<String>,
}
