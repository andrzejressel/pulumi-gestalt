#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LifecyclePolicyPolicyDetailsScheduleCrossRegionCopyRule {
    #[builder(into)]
    #[serde(rename = "cmkArn")]
    pub r#cmk_arn: Option<String>,
    #[builder(into)]
    #[serde(rename = "copyTags")]
    pub r#copy_tags: Option<bool>,
    #[builder(into)]
    #[serde(rename = "deprecateRule")]
    pub r#deprecate_rule: Option<Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleCrossRegionCopyRuleDeprecateRule>>,
    #[builder(into)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: bool,
    #[builder(into)]
    #[serde(rename = "retainRule")]
    pub r#retain_rule: Option<Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleCrossRegionCopyRuleRetainRule>>,
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: String,
}
