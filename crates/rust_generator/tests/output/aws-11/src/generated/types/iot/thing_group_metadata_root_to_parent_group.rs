#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThingGroupMetadataRootToParentGroup {
    #[builder(into)]
    #[serde(rename = "groupArn")]
    pub r#group_arn: Option<String>,
    #[builder(into)]
    #[serde(rename = "groupName")]
    pub r#group_name: Option<String>,
}
