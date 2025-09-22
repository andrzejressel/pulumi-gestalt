#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleOverrideAction {
    /// Override the rule action setting to count (i.e., only count matches). Configured as an empty block `{}`.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Option<Box<super::super::types::wafv2::WebAclRuleOverrideActionCount>>,
    /// Don't override the rule action setting. Configured as an empty block `{}`.
    #[builder(into)]
    #[serde(rename = "none")]
    pub r#none: Option<Box<super::super::types::wafv2::WebAclRuleOverrideActionNone>>,
}
