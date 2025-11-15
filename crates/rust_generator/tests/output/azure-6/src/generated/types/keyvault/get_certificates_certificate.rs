#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCertificatesCertificate {
    /// Whether this certificate is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// The ID of this certificate.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The name of certificate.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The tags of this certificate.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: std::collections::HashMap<String, String>,
}
