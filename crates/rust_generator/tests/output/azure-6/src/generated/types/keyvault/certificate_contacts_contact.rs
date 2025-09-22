#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateContactsContact {
    /// E-mail address of the contact.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: String,
    /// Name of the contact.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Phone number of the contact.
    #[builder(into)]
    #[serde(rename = "phone")]
    pub r#phone: Option<String>,
}
