#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SsisEnvironmentResponse {
    /// Metadata description.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Folder id which contains environment.
    #[builder(into)]
    #[serde(rename = "folderId")]
    pub r#folder_id: Option<f64>,
    /// Metadata id.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<f64>,
    /// Metadata name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The type of SSIS object metadata.
    /// Expected value is 'Environment'.
    #[builder(skip)]
    #[serde(rename = "type")]
    r#type_: super::constants::ConstStringEnvironment,
    /// Variable in environment
    #[builder(into)]
    #[serde(rename = "variables")]
    pub r#variables: Option<Vec<super::types::SsisVariableResponse>>,
}
