#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetUserHierarchyGroupHierarchyPathLevelOne {
    /// ARN of the hierarchy group.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// The identifier of the hierarchy group.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Returns information on a specific hierarchy group by name
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
