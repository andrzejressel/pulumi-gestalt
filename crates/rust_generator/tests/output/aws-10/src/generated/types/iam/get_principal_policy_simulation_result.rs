#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPrincipalPolicySimulationResult {
    /// The name of the single IAM action used for this particular request.
    #[builder(into)]
    #[serde(rename = "actionName")]
    pub r#action_name: String,
    /// `true` if `decision` is "allowed", and `false` otherwise.
    #[builder(into)]
    #[serde(rename = "allowed")]
    pub r#allowed: bool,
    /// The raw decision determined from all of the policies in scope; either "allowed", "explicitDeny", or "implicitDeny".
    #[builder(into)]
    #[serde(rename = "decision")]
    pub r#decision: String,
    /// A map of arbitrary metadata entries returned by the policy simulator for this request.
    #[builder(into)]
    #[serde(rename = "decisionDetails")]
    pub r#decision_details: std::collections::HashMap<String, String>,
    /// A nested set of objects describing which policies contained statements that were relevant to this simulation request. Each object has attributes `source_policy_id` and `source_policy_type` to identify one of the policies.
    #[builder(into)]
    #[serde(rename = "matchedStatements")]
    pub r#matched_statements: Vec<super::super::types::iam::GetPrincipalPolicySimulationResultMatchedStatement>,
    /// A set of context keys (or condition keys) that were needed by some of the policies contributing to this result but not specified using a `context` block in the configuration. Missing or incorrect context keys will typically cause a simulated request to be disallowed.
    #[builder(into)]
    #[serde(rename = "missingContextKeys")]
    pub r#missing_context_keys: Vec<String>,
    /// ARN of the resource that was used for this particular request. When you specify multiple actions and multiple resource ARNs, that causes a separate policy request for each combination of unique action and resource.
    #[builder(into)]
    #[serde(rename = "resourceArn")]
    pub r#resource_arn: String,
}
