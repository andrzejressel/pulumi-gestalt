#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetImageVersionsImageVersion {
    /// The string identifier of the image version, in the form: "composer-x.y.z-airflow-a.b.c"
    #[builder(into)]
    #[serde(rename = "imageVersionId")]
    pub r#image_version_id: String,
    /// Supported python versions for this image version
    #[builder(into)]
    #[serde(rename = "supportedPythonVersions")]
    pub r#supported_python_versions: Vec<String>,
}
