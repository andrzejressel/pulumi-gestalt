#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UptimeCheckConfigResourceGroup {
    /// The group of resources being monitored. Should be the `name` of a group
    #[builder(into)]
    #[serde(rename = "groupId")]
    pub r#group_id: Option<String>,
    /// The resource type of the group members.
    /// Possible values are: `RESOURCE_TYPE_UNSPECIFIED`, `INSTANCE`, `AWS_ELB_LOAD_BALANCER`.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Option<String>,
}
