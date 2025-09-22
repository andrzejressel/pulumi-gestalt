#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SsisFolderResponse {
    /// Metadata description.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Metadata id.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<f64>,
    /// Metadata name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The type of SSIS object metadata.
    /// Expected value is 'Folder'.
    #[builder(skip)]
    #[serde(rename = "type")]
    r#type_: super::constants::ConstStringFolder,
}
