#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSetOutputColumn {
    /// Field folder description.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Display name for the dataset.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
