#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SsisEnvironmentReferenceResponse {
    /// Environment folder name.
    #[builder(into)]
    #[serde(rename = "environmentFolderName")]
    pub r#environment_folder_name: Option<String>,
    /// Environment name.
    #[builder(into)]
    #[serde(rename = "environmentName")]
    pub r#environment_name: Option<String>,
    /// Environment reference id.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<f64>,
    /// Reference type
    #[builder(into)]
    #[serde(rename = "referenceType")]
    pub r#reference_type: Option<String>,
}
