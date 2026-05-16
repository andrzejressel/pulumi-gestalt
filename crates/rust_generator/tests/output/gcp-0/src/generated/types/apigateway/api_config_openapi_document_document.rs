#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApiConfigOpenapiDocumentDocument {
    /// Base64 encoded content of the file.
    #[builder(into)]
    #[serde(rename = "contents")]
    pub r#contents: String,
    /// The file path (full or relative path). This is typically the path of the file when it is uploaded.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
}
