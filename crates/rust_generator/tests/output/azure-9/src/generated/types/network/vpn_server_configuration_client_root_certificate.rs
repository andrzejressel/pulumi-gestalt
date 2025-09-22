#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VpnServerConfigurationClientRootCertificate {
    /// A name used to uniquely identify this certificate.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The Public Key Data associated with the Certificate.
    #[builder(into)]
    #[serde(rename = "publicCertData")]
    pub r#public_cert_data: String,
}
