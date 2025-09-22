#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualMachineOsProfileSecret {
    /// Specifies the ID of the Key Vault to use.
    #[builder(into)]
    #[serde(rename = "sourceVaultId")]
    pub r#source_vault_id: String,
    /// One or more `vault_certificates` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "vaultCertificates")]
    pub r#vault_certificates: Option<Vec<super::super::types::compute::VirtualMachineOsProfileSecretVaultCertificate>>,
}
