#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkInsightsAnalysisForwardPathComponentAclRule {
    #[builder(into)]
    #[serde(rename = "cidr")]
    pub r#cidr: Option<String>,
    #[builder(into)]
    #[serde(rename = "egress")]
    pub r#egress: Option<bool>,
    #[builder(into)]
    #[serde(rename = "portRanges")]
    pub r#port_ranges: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentAclRulePortRange>>,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
    #[builder(into)]
    #[serde(rename = "ruleAction")]
    pub r#rule_action: Option<String>,
    #[builder(into)]
    #[serde(rename = "ruleNumber")]
    pub r#rule_number: Option<i32>,
}
