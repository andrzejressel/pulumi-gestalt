#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CompositePathResponse {
    /// Sort order for composite paths.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Option<String>,
    /// The path for which the indexing behavior applies to. Index paths typically start with root and end with wildcard (/path/*)
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
}
