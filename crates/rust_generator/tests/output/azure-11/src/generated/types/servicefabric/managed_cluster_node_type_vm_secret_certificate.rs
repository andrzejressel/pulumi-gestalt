#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagedClusterNodeTypeVmSecretCertificate {
    /// The certificate store on the Virtual Machine to which the certificate should be added.
    #[builder(into)]
    #[serde(rename = "store")]
    pub r#store: String,
    /// The URL of a certificate that has been uploaded to Key Vault as a secret
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}
