#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SsisPackageResponse {
    /// Metadata description.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Folder id which contains package.
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
    /// Parameters in package
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<Vec<super::types::SsisParameterResponse>>,
    /// Project id which contains package.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Option<f64>,
    /// Project version which contains package.
    #[builder(into)]
    #[serde(rename = "projectVersion")]
    pub r#project_version: Option<f64>,
    /// The type of SSIS object metadata.
    /// Expected value is 'Package'.
    #[builder(skip)]
    #[serde(rename = "type")]
    r#type_: super::constants::ConstStringPackage,
}
