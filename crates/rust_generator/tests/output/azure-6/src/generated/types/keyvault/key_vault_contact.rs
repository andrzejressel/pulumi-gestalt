#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KeyVaultContact {
    /// E-mail address of the contact.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Box<String>,
    /// Name of the contact.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Phone number of the contact.
    #[builder(into, default)]
    #[serde(rename = "phone")]
    pub r#phone: Box<Option<String>>,
}
