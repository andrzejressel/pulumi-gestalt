#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MetadataSource {
    /// The id of the content source, the solution ID, Log Analytics Workspace name etc.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The kind of the content source. Possible values are `Community`, `LocalWorkspace`, `Solution` and `SourceRepository`.
    #[builder(into)]
    #[serde(rename = "kind")]
    pub r#kind: String,
    /// The name of the content source, repo name, solution name, Log Analytics Workspace name, etc.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}
