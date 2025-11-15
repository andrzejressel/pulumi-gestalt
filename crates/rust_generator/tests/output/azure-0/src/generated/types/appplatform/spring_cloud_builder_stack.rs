#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudBuilderStack {
    /// Specifies the ID of the ClusterStack.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Specifies the version of the ClusterStack
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}
