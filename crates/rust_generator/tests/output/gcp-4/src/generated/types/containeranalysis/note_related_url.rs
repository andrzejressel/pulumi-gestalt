#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NoteRelatedUrl {
    /// Label to describe usage of the URL
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Option<String>,
    /// Specific URL associated with the resource.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}
