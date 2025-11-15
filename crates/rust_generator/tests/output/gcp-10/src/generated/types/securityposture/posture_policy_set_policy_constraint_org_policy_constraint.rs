#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PosturePolicySetPolicyConstraintOrgPolicyConstraint {
    /// Organization policy canned constraint Id
    #[builder(into)]
    #[serde(rename = "cannedConstraintId")]
    pub r#canned_constraint_id: String,
    /// Definition of policy rules
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "policyRules")]
    pub r#policy_rules: Vec<super::super::types::securityposture::PosturePolicySetPolicyConstraintOrgPolicyConstraintPolicyRule>,
}
