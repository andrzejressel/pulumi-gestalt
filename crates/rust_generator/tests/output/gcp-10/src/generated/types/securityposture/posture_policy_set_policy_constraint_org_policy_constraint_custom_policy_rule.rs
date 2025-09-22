#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PosturePolicySetPolicyConstraintOrgPolicyConstraintCustomPolicyRule {
    /// Setting this to true means that all values are allowed. This field can be set only in policies for list constraints.
    #[builder(into)]
    #[serde(rename = "allowAll")]
    pub r#allow_all: Option<bool>,
    /// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language.
    /// This page details the objects and attributes that are used to the build the CEL expressions for
    /// custom access levels - https://cloud.google.com/access-context-manager/docs/custom-access-level-spec.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "condition")]
    pub r#condition: Option<Box<super::super::types::securityposture::PosturePolicySetPolicyConstraintOrgPolicyConstraintCustomPolicyRuleCondition>>,
    /// Setting this to true means that all values are denied. This field can be set only in policies for list constraints.
    #[builder(into)]
    #[serde(rename = "denyAll")]
    pub r#deny_all: Option<bool>,
    /// If `true`, then the policy is enforced. If `false`, then any configuration is acceptable.
    /// This field can be set only in policies for boolean constraints.
    #[builder(into)]
    #[serde(rename = "enforce")]
    pub r#enforce: Option<bool>,
    /// List of values to be used for this policy rule. This field can be set only in policies for list constraints.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Option<Box<super::super::types::securityposture::PosturePolicySetPolicyConstraintOrgPolicyConstraintCustomPolicyRuleValues>>,
}
