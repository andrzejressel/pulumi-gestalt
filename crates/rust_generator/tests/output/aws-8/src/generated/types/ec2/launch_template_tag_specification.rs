#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LaunchTemplateTagSpecification {
    /// The type of resource to tag.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Option<String>,
    /// A map of tags to assign to the resource.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
}
