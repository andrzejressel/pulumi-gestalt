#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SsisEnvironmentResponse {
    /// Metadata description.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Folder id which contains environment.
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
    /// The type of SSIS object metadata.
    /// Expected value is 'Environment'.
    #[builder(skip)]
    #[serde(rename = "type")]
    r#type_: Box<super::constants::ConstStringEnvironment>,
    /// Variable in environment
    #[builder(into, default)]
    #[serde(rename = "variables")]
    pub r#variables: Box<Option<Vec<super::types::SsisVariableResponse>>>,
}
