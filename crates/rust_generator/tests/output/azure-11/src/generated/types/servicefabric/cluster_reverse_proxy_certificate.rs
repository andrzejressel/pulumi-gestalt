#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterReverseProxyCertificate {
    /// The Thumbprint of the Certificate.
    #[builder(into)]
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: String,
    /// The Secondary Thumbprint of the Certificate.
    #[builder(into)]
    #[serde(rename = "thumbprintSecondary")]
    pub r#thumbprint_secondary: Option<String>,
    /// The X509 Store where the Certificate Exists, such as `My`.
    #[builder(into)]
    #[serde(rename = "x509StoreName")]
    pub r#x_509_store_name: String,
}
