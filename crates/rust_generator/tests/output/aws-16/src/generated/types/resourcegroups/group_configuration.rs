#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GroupConfiguration {
    /// A collection of parameters for this group configuration item. See below for details.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<Vec<super::super::types::resourcegroups::GroupConfigurationParameter>>>,
    /// Specifies the type of group configuration item.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
