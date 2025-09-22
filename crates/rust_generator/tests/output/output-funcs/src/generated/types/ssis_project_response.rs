#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SsisProjectResponse {
    /// Metadata description.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Environment reference in project
    #[builder(into)]
    #[serde(rename = "environmentRefs")]
    pub r#environment_refs: Option<Vec<super::types::SsisEnvironmentReferenceResponse>>,
    /// Folder id which contains project.
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
    /// Parameters in project
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<Vec<super::types::SsisParameterResponse>>,
    /// The type of SSIS object metadata.
    /// Expected value is 'Project'.
    #[builder(skip)]
    #[serde(rename = "type")]
    r#type_: super::constants::ConstStringProject,
    /// Project version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<f64>,
}
