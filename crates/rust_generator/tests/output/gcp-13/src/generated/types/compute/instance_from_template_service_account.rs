#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceFromTemplateServiceAccount {
    /// The service account e-mail address.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// A list of service scopes.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Vec<String>,
}
