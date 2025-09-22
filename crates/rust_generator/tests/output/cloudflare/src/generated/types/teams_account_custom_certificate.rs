#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TeamsAccountCustomCertificate {
    /// Whether TLS encryption should use a custom certificate.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// ID of custom certificate.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[builder(into)]
    #[serde(rename = "updatedAt")]
    pub r#updated_at: Option<String>,
}
