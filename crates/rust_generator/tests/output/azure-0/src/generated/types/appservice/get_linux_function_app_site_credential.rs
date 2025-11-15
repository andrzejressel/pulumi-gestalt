#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLinuxFunctionAppSiteCredential {
    /// The name which should be used for this Linux Function App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The Site Credentials Password used for publishing.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
}
