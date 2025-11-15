#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApiContact {
    /// The email address of the contact person/organization.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// The name of the contact person/organization.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Absolute URL of the contact information.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}
