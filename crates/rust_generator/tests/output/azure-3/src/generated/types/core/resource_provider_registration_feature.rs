#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResourceProviderRegistrationFeature {
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Should this feature be Registered or Unregistered?
    #[builder(into)]
    #[serde(rename = "registered")]
    pub r#registered: bool,
}
