#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PosturePolicySetPolicyConstraint {
    /// Organization policy canned constraint definition.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "orgPolicyConstraint")]
    pub r#org_policy_constraint: Option<Box<super::super::types::securityposture::PosturePolicySetPolicyConstraintOrgPolicyConstraint>>,
    /// Organization policy custom constraint policy definition.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "orgPolicyConstraintCustom")]
    pub r#org_policy_constraint_custom: Option<Box<super::super::types::securityposture::PosturePolicySetPolicyConstraintOrgPolicyConstraintCustom>>,
    /// Definition of Security Health Analytics Custom Module.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "securityHealthAnalyticsCustomModule")]
    pub r#security_health_analytics_custom_module: Option<Box<super::super::types::securityposture::PosturePolicySetPolicyConstraintSecurityHealthAnalyticsCustomModule>>,
    /// Security Health Analytics built-in detector definition.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "securityHealthAnalyticsModule")]
    pub r#security_health_analytics_module: Option<Box<super::super::types::securityposture::PosturePolicySetPolicyConstraintSecurityHealthAnalyticsModule>>,
}
