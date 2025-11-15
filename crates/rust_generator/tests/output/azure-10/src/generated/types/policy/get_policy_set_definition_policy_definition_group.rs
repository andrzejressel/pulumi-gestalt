#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPolicySetDefinitionPolicyDefinitionGroup {
    /// The ID of a resource that contains additional metadata about this policy definition group.
    #[builder(into)]
    #[serde(rename = "additionalMetadataResourceId")]
    pub r#additional_metadata_resource_id: String,
    /// The category of this policy definition group.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: String,
    /// The description of this policy definition group.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// Specifies the display name of the Policy Set Definition. Conflicts with `name`.
    /// 
    /// **NOTE** As `display_name` is not unique errors may occur when there are multiple policy set definitions with same display name.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: String,
    /// Specifies the name of the Policy Set Definition. Conflicts with `display_name`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
