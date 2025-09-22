#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPolicyDocumentStatement {
    /// List of actions that this statement either allows or denies. For example, `["ec2:RunInstances", "s3:*"]`.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Option<Vec<String>>,
    /// Configuration block for a condition. Detailed below.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Option<Vec<super::super::types::iam::GetPolicyDocumentStatementCondition>>,
    /// Whether this statement allows or denies the given actions. Valid values are `Allow` and `Deny`. Defaults to `Allow`.
    #[builder(into)]
    #[serde(rename = "effect")]
    pub r#effect: Option<String>,
    /// List of actions that this statement does *not* apply to. Use to apply a policy statement to all actions *except* those listed.
    #[builder(into)]
    #[serde(rename = "notActions")]
    pub r#not_actions: Option<Vec<String>>,
    /// Like `principals` except these are principals that the statement does *not* apply to.
    #[builder(into)]
    #[serde(rename = "notPrincipals")]
    pub r#not_principals: Option<Vec<super::super::types::iam::GetPolicyDocumentStatementNotPrincipal>>,
    /// List of resource ARNs that this statement does *not* apply to. Use to apply a policy statement to all resources *except* those listed. Conflicts with `resources`.
    #[builder(into)]
    #[serde(rename = "notResources")]
    pub r#not_resources: Option<Vec<String>>,
    /// Configuration block for principals. Detailed below.
    #[builder(into)]
    #[serde(rename = "principals")]
    pub r#principals: Option<Vec<super::super::types::iam::GetPolicyDocumentStatementPrincipal>>,
    /// List of resource ARNs that this statement applies to. This is required by AWS if used for an IAM policy. Conflicts with `not_resources`.
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Option<Vec<String>>,
    /// Sid (statement ID) is an identifier for a policy statement.
    #[builder(into)]
    #[serde(rename = "sid")]
    pub r#sid: Option<String>,
}
