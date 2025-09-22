#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLifecyclePolicyDocumentRule {
    /// Specifies the action type.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Option<Box<super::super::types::ecr::GetLifecyclePolicyDocumentRuleAction>>,
    /// Describes the purpose of a rule within a lifecycle policy.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Sets the order in which rules are evaluated, lowest to highest. When you add rules to a lifecycle policy, you must give them each a unique value for `priority`. Values do not need to be sequential across rules in a policy. A rule with a `tag_status` value of "any" must have the highest value for `priority` and be evaluated last.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// Collects parameters describing the selection criteria for the ECR lifecycle policy:
    #[builder(into)]
    #[serde(rename = "selection")]
    pub r#selection: Option<Box<super::super::types::ecr::GetLifecyclePolicyDocumentRuleSelection>>,
}
