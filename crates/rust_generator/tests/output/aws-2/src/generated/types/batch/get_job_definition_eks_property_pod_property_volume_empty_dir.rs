#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionEksPropertyPodPropertyVolumeEmptyDir {
    /// The medium to store the volume.
    #[builder(into)]
    #[serde(rename = "medium")]
    pub r#medium: String,
    /// The maximum size of the volume. By default, there's no maximum size defined.
    #[builder(into)]
    #[serde(rename = "sizeLimit")]
    pub r#size_limit: String,
}
