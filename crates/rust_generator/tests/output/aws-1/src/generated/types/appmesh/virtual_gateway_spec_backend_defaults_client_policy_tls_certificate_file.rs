#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateFile {
    /// Certificate trust chain for a certificate stored on the file system of the mesh endpoint that the proxy is running on. Must be between 1 and 255 characters in length.
    #[builder(into)]
    #[serde(rename = "certificateChain")]
    pub r#certificate_chain: String,
    /// Private key for a certificate stored on the file system of the mesh endpoint that the proxy is running on. Must be between 1 and 255 characters in length.
    #[builder(into)]
    #[serde(rename = "privateKey")]
    pub r#private_key: String,
}
