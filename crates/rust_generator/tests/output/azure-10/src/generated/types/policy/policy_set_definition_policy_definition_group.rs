#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicySetDefinitionPolicyDefinitionGroup {
    /// The ID of a resource that contains additional metadata about this policy definition group.
    #[builder(into)]
    #[serde(rename = "additionalMetadataResourceId")]
    pub r#additional_metadata_resource_id: Option<String>,
    /// The category of this policy definition group.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: Option<String>,
    /// The description of this policy definition group.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The display name of this policy definition group.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// The name of this policy definition group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
