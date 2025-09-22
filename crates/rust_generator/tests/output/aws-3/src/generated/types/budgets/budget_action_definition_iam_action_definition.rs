#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BudgetActionDefinitionIamActionDefinition {
    /// A list of groups to be attached. There must be at least one group.
    #[builder(into)]
    #[serde(rename = "groups")]
    pub r#groups: Option<Vec<String>>,
    /// The Amazon Resource Name (ARN) of the policy to be attached.
    #[builder(into)]
    #[serde(rename = "policyArn")]
    pub r#policy_arn: String,
    /// A list of roles to be attached. There must be at least one role.
    #[builder(into)]
    #[serde(rename = "roles")]
    pub r#roles: Option<Vec<String>>,
    /// A list of users to be attached. There must be at least one user.
    #[builder(into)]
    #[serde(rename = "users")]
    pub r#users: Option<Vec<String>>,
}
