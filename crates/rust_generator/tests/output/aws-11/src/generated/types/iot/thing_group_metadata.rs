#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ThingGroupMetadata {
    #[builder(into)]
    #[serde(rename = "creationDate")]
    pub r#creation_date: Option<String>,
    /// The name of the parent Thing Group.
    #[builder(into)]
    #[serde(rename = "parentGroupName")]
    pub r#parent_group_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "rootToParentGroups")]
    pub r#root_to_parent_groups: Option<Vec<super::super::types::iot::ThingGroupMetadataRootToParentGroup>>,
}
