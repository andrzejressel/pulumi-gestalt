#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AssignmentDynamicScopeFilter {
    /// Specifies a list of locations to scope the query to.
    #[builder(into)]
    #[serde(rename = "locations")]
    pub r#locations: Option<Vec<String>>,
    /// Specifies a list of allowed operating systems.
    #[builder(into)]
    #[serde(rename = "osTypes")]
    pub r#os_types: Option<Vec<String>>,
    /// Specifies a list of allowed resource groups.
    #[builder(into)]
    #[serde(rename = "resourceGroups")]
    pub r#resource_groups: Option<Vec<String>>,
    /// Specifies a list of allowed resources.
    #[builder(into)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Option<Vec<String>>,
    /// Filter VMs by `Any` or `All` specified tags. Defaults to `Any`.
    #[builder(into)]
    #[serde(rename = "tagFilter")]
    pub r#tag_filter: Option<String>,
    /// A mapping of tags for the VM
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Vec<super::super::types::maintenance::AssignmentDynamicScopeFilterTag>>,
}
