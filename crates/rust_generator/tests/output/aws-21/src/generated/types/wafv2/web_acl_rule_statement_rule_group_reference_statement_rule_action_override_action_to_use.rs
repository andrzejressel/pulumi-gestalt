#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUse {
    #[builder(into)]
    #[serde(rename = "allow")]
    pub r#allow: Option<Box<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUseAllow>>,
    #[builder(into)]
    #[serde(rename = "block")]
    pub r#block: Option<Box<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUseBlock>>,
    /// Instructs AWS WAF to run a Captcha check against the web request. See `captcha` below for details.
    #[builder(into)]
    #[serde(rename = "captcha")]
    pub r#captcha: Option<Box<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUseCaptcha>>,
    /// Instructs AWS WAF to run a check against the request to verify that the request is coming from a legitimate client session. See `challenge` below for details.
    #[builder(into)]
    #[serde(rename = "challenge")]
    pub r#challenge: Option<Box<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUseChallenge>>,
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Option<Box<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUseCount>>,
}
