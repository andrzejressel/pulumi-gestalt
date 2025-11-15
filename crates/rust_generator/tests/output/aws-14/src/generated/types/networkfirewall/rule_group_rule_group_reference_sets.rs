#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleGroupRuleGroupReferenceSets {
    #[builder(into)]
    #[serde(rename = "ipSetReferences")]
    pub r#ip_set_references: Option<Vec<super::super::types::networkfirewall::RuleGroupRuleGroupReferenceSetsIpSetReference>>,
}
