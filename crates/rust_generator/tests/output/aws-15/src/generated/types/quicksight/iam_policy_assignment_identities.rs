#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IamPolicyAssignmentIdentities {
    /// Array of Quicksight group names to assign the policy to.
    #[builder(into)]
    #[serde(rename = "groups")]
    pub r#groups: Option<Vec<String>>,
    /// Array of Quicksight user names to assign the policy to.
    #[builder(into)]
    #[serde(rename = "users")]
    pub r#users: Option<Vec<String>>,
}
