#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetUserHierarchyStructureHierarchyStructureLevelTwo {
    /// ARN of the hierarchy level.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// The identifier of the hierarchy level.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Name of the user hierarchy level. Must not be more than 50 characters.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
