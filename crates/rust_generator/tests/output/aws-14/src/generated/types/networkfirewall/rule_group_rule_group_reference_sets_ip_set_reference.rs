#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuleGroupRuleGroupReferenceSetsIpSetReference {
    /// Set of configuration blocks that define the IP Reference information. See IP Set Reference below for details.
    #[builder(into)]
    #[serde(rename = "ipSetReferences")]
    pub r#ip_set_references: Box<Vec<super::super::types::networkfirewall::RuleGroupRuleGroupReferenceSetsIpSetReferenceIpSetReference>>,
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
}
