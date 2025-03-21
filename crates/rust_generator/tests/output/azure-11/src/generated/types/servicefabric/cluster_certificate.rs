#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterCertificate {
    /// The Thumbprint of the Certificate.
    #[builder(into)]
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: Box<String>,
    /// The Secondary Thumbprint of the Certificate.
    #[builder(into, default)]
    #[serde(rename = "thumbprintSecondary")]
    pub r#thumbprint_secondary: Box<Option<String>>,
    /// The X509 Store where the Certificate Exists, such as `My`.
    #[builder(into)]
    #[serde(rename = "x509StoreName")]
    pub r#x_509_store_name: Box<String>,
}
