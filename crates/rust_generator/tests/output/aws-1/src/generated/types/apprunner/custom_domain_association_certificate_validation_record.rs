#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomDomainAssociationCertificateValidationRecord {
    /// Certificate CNAME record name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Current state of the certificate CNAME record validation. It should change to `SUCCESS` after App Runner completes validation with your DNS.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// Record type, always `CNAME`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    /// Certificate CNAME record value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
