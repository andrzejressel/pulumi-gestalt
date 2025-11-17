#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetListenerRuleActionForward {
    /// Target group stickiness for the rule.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "stickiness")]
    pub r#stickiness: Option<Box<super::super::types::lb::GetListenerRuleActionForwardStickiness>>,
    /// Set of target groups for the action.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "targetGroups")]
    pub r#target_groups: Option<Vec<super::super::types::lb::GetListenerRuleActionForwardTargetGroup>>,
}
