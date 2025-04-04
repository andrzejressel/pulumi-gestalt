#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SsisProjectResponse {
    /// Metadata description.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Environment reference in project
    #[builder(into, default)]
    #[serde(rename = "environmentRefs")]
    pub r#environment_refs: Box<Option<Vec<super::types::SsisEnvironmentReferenceResponse>>>,
    /// Folder id which contains project.
    #[builder(into, default)]
    #[serde(rename = "folderId")]
    pub r#folder_id: Box<Option<f64>>,
    /// Metadata id.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<f64>>,
    /// Metadata name.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Parameters in project
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<Vec<super::types::SsisParameterResponse>>>,
    /// The type of SSIS object metadata.
    /// Expected value is 'Project'.
    #[builder(skip)]
    #[serde(rename = "type")]
    r#type_: Box<super::constants::ConstStringProject>,
    /// Project version.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<f64>>,
}
