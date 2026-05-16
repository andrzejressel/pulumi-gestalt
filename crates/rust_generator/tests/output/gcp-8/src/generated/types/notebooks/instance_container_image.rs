#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceContainerImage {
    /// The path to the container image repository.
    /// For example: gcr.io/{project_id}/{imageName}
    #[builder(into)]
    #[serde(rename = "repository")]
    pub r#repository: String,
    /// The tag of the container image. If not specified, this defaults to the latest tag.
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Option<String>,
}
